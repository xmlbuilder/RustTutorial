
use crate::doe::anom::AnomOptions;
use crate::doe::doe_anom_response::{build_anom_for_all_factors, FactorAnomResult};
use crate::doe::orthogonal_array::{build_design_from_orthogonal_array_for_factors, FactorLevels, OrthogonalArray};
use crate::doe::response_surface_quadratic::ResponseSurfaceQuadratic;

/// Full DOE analysis result: quadratic RS + ANOM for each factor.
#[derive(Clone, Debug)]
pub struct DoeFullAnalysis {
    pub rs_model: ResponseSurfaceQuadratic,
    pub factor_anoms: Vec<FactorAnomResult>,
}

/// Run full DOE analysis:
/// - build RS design for selected factors
/// - fit quadratic response surface
/// - build ANOM for all factors
pub fn run_doe_full_analysis(
    oa: &OrthogonalArray,
    all_levels: &[FactorLevels],
    factor_indices_for_rs: &[usize],
    y: &[f64],
    factor_names: &[String],
    anom_opt: &AnomOptions,
) -> Result<DoeFullAnalysis, String> {
    if y.len() != oa.runs {
        return Err("run_doe_full_analysis: y length mismatch with OA runs".into());
    }

    // Build design for RS
    let design =
        build_design_from_orthogonal_array_for_factors(oa, all_levels, factor_indices_for_rs);

    // Fit quadratic RS
    let mut rs = ResponseSurfaceQuadratic::new();
    if !rs.fit(&design, y) {
        return Err("run_doe_full_analysis: ResponseSurfaceQuadratic::fit failed".into());
    }

    // ANOM for all factors
    let factor_anoms = build_anom_for_all_factors(oa, y, factor_names, anom_opt);

    Ok(DoeFullAnalysis {
        rs_model: rs,
        factor_anoms,
    })
}
