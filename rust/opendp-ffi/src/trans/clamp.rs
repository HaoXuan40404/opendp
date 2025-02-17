use std::convert::TryFrom;
use std::ops::Bound;
use std::os::raw::c_char;

use opendp::err;
use opendp::traits::{CheckNull, TotalOrd};
use opendp::trans::{make_clamp, make_unclamp};

use crate::any::{AnyTransformation, AnyObject, Downcast};
use crate::core::{FfiResult, IntoAnyTransformationFfiResultExt};
use crate::util::Type;

#[no_mangle]
pub extern "C" fn opendp_trans__make_clamp(
    bounds: *const AnyObject,
    TA: *const c_char,
) -> FfiResult<*mut AnyTransformation> {
    let TA = try_!(Type::try_from(TA));

    fn monomorphize_dataset<TA>(bounds: *const AnyObject) -> FfiResult<*mut AnyTransformation>
        where TA: 'static + Clone + TotalOrd + CheckNull {
        let bounds = try_!(try_as_ref!(bounds).downcast_ref::<(TA, TA)>()).clone();
        make_clamp::<TA>(bounds).into_any()
    }
    dispatch!(monomorphize_dataset, [
        (TA, @numbers)
    ], (bounds))
}


#[no_mangle]
pub extern "C" fn opendp_trans__make_unclamp(
    bounds: *const AnyObject,
    TA: *const c_char
) -> FfiResult<*mut AnyTransformation> {
    let TA = try_!(Type::try_from(TA));
    fn monomorphize_dataset<TA>(bounds: *const AnyObject) -> FfiResult<*mut AnyTransformation>
        where TA: 'static + Clone + TotalOrd + CheckNull {
        let (lower, upper) = try_!(try_as_ref!(bounds).downcast_ref::<(TA, TA)>()).clone();
        make_unclamp::<TA>((Bound::Included(lower), Bound::Included(upper))).into_any()
    }
    dispatch!(monomorphize_dataset, [
        (TA, @numbers)
    ], (bounds))
}


#[cfg(test)]
mod tests {

    use opendp::error::Fallible;

    use crate::any::{AnyObject, Downcast};
    use crate::core;
    use crate::trans::clamp::opendp_trans__make_clamp;
    use crate::util;
    use crate::util::ToCharP;

    #[test]
    fn test_make_vector_clamp() -> Fallible<()> {
        let transformation = Result::from(opendp_trans__make_clamp(
            util::into_raw(AnyObject::new((0.0, 10.0))),
            "f64".to_char_p(),
        ))?;
        let arg = AnyObject::new_raw(vec![-1.0, 5.0, 11.0]);
        let res = core::opendp_core__transformation_invoke(&transformation, arg);
        let res: Vec<f64> = Fallible::from(res)?.downcast()?;
        assert_eq!(res, vec![0.0, 5.0, 10.0]);
        Ok(())
    }
}
