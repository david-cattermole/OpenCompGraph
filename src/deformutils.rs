use log::{debug, error, info, warn};

use crate::deformer::brownian;
use crate::deformer::Deformer;
use crate::deformer::DeformerType;

pub fn apply_deformers(deformers: &Vec<Deformer>, buffer: &mut [f32]) {
    for deformer in deformers {
        debug!("apply_deformer...");
        let enable = deformer.get_attr_i32("enable");
        if enable == 0 {
            continue;
        }

        let deformer_type = deformer.deformer_type();
        debug!("Deformer Type: {:?}", deformer_type);
        match deformer_type {
            DeformerType::Brownian => {
                let k1 = deformer.get_attr_f32("k1");
                let k2 = deformer.get_attr_f32("k2");
                let xc = deformer.get_attr_f32("center_x");
                let yc = deformer.get_attr_f32("center_y");
                let parameters = brownian::Parameters::new(xc, yc, k1, k2);
                brownian::deform_slice_by_3_in_place(buffer, parameters)
            }
            DeformerType::Null => (),
            DeformerType::Unknown => warn!("Unknown deformer type."),
        }
    }
}
