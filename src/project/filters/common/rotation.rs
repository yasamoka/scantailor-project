use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rotation {
    #[serde(rename(serialize = "@degrees"))]
    pub degrees: Degrees,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Degrees {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "90")]
    _90,
    #[serde(rename = "180")]
    _180,
    #[serde(rename = "270")]
    _270,
}

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;

    use crate::project::{filters::common::rotation::Degrees, ProjectXML};

    #[test]
    fn it_deserializes_rotation_degrees() {
        const _0: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const _90: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image id="3"><rotation degrees="90"/></image><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="4961" y="0"/><point x="4961" y="7016"/><point x="0" y="7016"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="90"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="90"/><page-outline><point x="0" y="0"/><point x="4961" y="0"/><point x="4961" y="7016"/><point x="0" y="7016"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="192"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const _180: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image id="3"><rotation degrees="180"/></image><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="180"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="180"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const _270: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image id="3"><rotation degrees="270"/></image><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="4961" y="0"/><point x="4961" y="7016"/><point x="0" y="7016"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="270"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="270"/><page-outline><point x="0" y="0"/><point x="4961" y="0"/><point x="4961" y="7016"/><point x="0" y="7016"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="192"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;

        for (content, expected_degrees) in [
            (_0, Degrees::_0),
            (_90, Degrees::_90),
            (_180, Degrees::_180),
            (_270, Degrees::_270),
        ] {
            let project: ProjectXML = from_str(content).unwrap();

            let mut images = project.filters.page_split.image.unwrap();
            assert_eq!(images.len(), 1);

            let image = images.pop().unwrap();
            assert_eq!(image.params.dependencies.rotation.degrees, expected_degrees);
        }
    }
}
