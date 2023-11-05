use std::fmt;

use crate::constants::gibbet_phases;

pub enum Gibbet {
    Gibbet,
    Head,
    HeadAndRightArm,
    HeadRightArmAndLeftArm,
    HeadRightArmLeftArmAndStem,
    HeadRightArmLeftArmStemAndRightLeg,
    HeadRightArmLeftArmStemRightLegAndLeftLeg,
}

impl fmt::Display for Gibbet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Gibbet => gibbet_phases::GIBBET,
                Self::Head => gibbet_phases::HEAD,
                Self::HeadAndRightArm => gibbet_phases::HEAD_AND_RIGHT_ARM,
                Self::HeadRightArmAndLeftArm => gibbet_phases::HEAD_RIGHT_ARM_AND_LEFT_ARM,
                Self::HeadRightArmLeftArmAndStem => gibbet_phases::HEAD_RIGHT_ARM_LEFT_ARM_AND_STEM,
                Self::HeadRightArmLeftArmStemAndRightLeg => gibbet_phases::HEAD_RIGHT_ARM_LEFT_ARM_STEM_AND_RIGHT_LEG,
                Self::HeadRightArmLeftArmStemRightLegAndLeftLeg => gibbet_phases::HEAD_RIGHT_ARM_LEFT_ARM_STEM_RIGHT_LEG_AND_LEFT_LEG,
            }
        )
    }
}

