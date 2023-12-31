use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    directories::Directories, file_name_disambiguation::FileNameDisambiguation, files::Files,
    filters::Filters, images::Images, layout_direction::LayoutDirection, pages::Pages,
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename = "project")]
pub struct ProjectXML {
    #[serde(rename(serialize = "@layoutDirection", deserialize = "layoutDirection"))]
    pub layout_direction: LayoutDirection,

    #[serde(rename(serialize = "@outputDirectory", deserialize = "outputDirectory"))]
    pub output_dir: PathBuf,

    #[serde(rename(serialize = "@version"))]
    #[validate(range(min = 3, max = 3))]
    pub version: u8,

    #[validate]
    pub directories: Directories,

    pub files: Files,

    pub images: Images,

    pub pages: Pages,

    #[serde(rename = "file-name-disambiguation")]
    pub file_name_disambiguation: FileNameDisambiguation,

    #[validate]
    pub filters: Filters,
}

#[cfg(test)]
mod test {
    use quick_xml::se::to_string;
    use serde_xml_rs::from_str;
    use validator::Validate;

    use super::ProjectXML;

    const ORIGINAL_CONTENT: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="202311050607_0001.jpg"/><file dirId="1" id="5" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image><image fileId="5" fileImage="0" id="6" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/><page id="7" imageId="6" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/><mapping file="5" label="0"/></file-name-disambiguation><filters><fix-orientation><image id="3"><rotation degrees="90"/></image><image id="6"><rotation degrees="90"/></image><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page><page id="7"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-cut"><params mode="manual"><pages type="single-cut"><outline><point x="0" y="0"/><point x="4961" y="0"/><point x="4961" y="7016"/><point x="0" y="7016"/><point x="0" y="0"/></outline><cutter1><p1 x="1142.578008915305" y="66.34323922734029"/><p2 x="1142.578008915305" y="6964.399702823181"/></cutter1><cutter2><p1 x="4959.99008" y="0"/><p2 x="4959.99008" y="7015.985967999999"/></cutter2></pages><dependencies><rotation degrees="90"/><size height="4961" width="7016"/><layoutType>single-cut</layoutType></dependencies></params></image><image id="6" layoutType="single-cut"><params mode="manual"><pages type="single-cut"><outline><point x="0" y="0"/><point x="4961" y="0"/><point x="4961" y="7016"/><point x="0" y="7016"/><point x="0" y="0"/></outline><cutter1><p1 x="1149.949479940565" y="206.4011887072809"/><p2 x="1149.949479940565" y="7104.457652303121"/></cutter1><cutter2><p1 x="4959.99008" y="0"/><p2 x="4959.99008" y="7015.985967999999"/></cutter2></pages><dependencies><rotation degrees="90"/><size height="4961" width="7016"/><layoutType>single-cut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="90"/><page-outline><point x="1143" y="0"/><point x="4960" y="0"/><point x="4960" y="7016"/><point x="1143" y="7016"/><point x="1143" y="0"/></page-outline></dependencies></params></page><page id="7"><params angle="0.125" mode="auto"><dependencies><rotation degrees="90"/><page-outline><point x="1150" y="0"/><point x="4960" y="0"/><point x="4960" y="7016"/><point x="1150" y="7016"/><point x="1150" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="190"/></page><page id="7"><image-params blackOnWhite="1" bwThreshold="190"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="2236" width="3328" x="116" y="364"/><page-rect height="7024.310698751963" width="3832.297441614658" x="0" y="0"/><content-size-mm height="94.65752264837863" width="140.8856151045636"/><dependencies><rotated-page-outline><point x="15.30652539770676" y="0"/><point x="3832.297441614658" y="8.32739558766345"/><point x="3816.990916216952" y="7024.310698751963"/><point x="0" y="7015.9833031643"/><point x="15.30652539770676" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page><page id="7"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="2240" width="3328" x="108" y="364"/><page-rect height="7024.295427133123" width="3825.297458273417" x="0" y="0"/><content-size-mm height="94.82685632037932" width="140.8856151045636"/><dependencies><rotated-page-outline><point x="15.30652539770676" y="0"/><point x="3825.297458273417" y="8.31212396882309"/><point x="3809.99093287571" y="7024.295427133123"/><point x="0" y="7015.9833031643"/><point x="15.30652539770676" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="5" right="5" top="5"/><pageRect height="7024.310698751963" width="3832.297441614658" x="0" y="0"/><contentRect height="2236" width="3328" x="116" y="364"/><contentSizeMM height="94.65752264837863" width="140.8856151045636"/><alignment hor="center" null="0" vert="center"/></params></page><page id="7"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="7024.295427133123" width="3825.297458273417" x="0" y="0"/><contentRect height="2240" width="3328" x="108" y="364"/><contentSizeMM height="94.82685632037932" width="140.8856151045636"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="colorOrGray"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/><output-params><image blackOnWhite="1" depthPerception="2" despeckleLevel="1"><size height="2476" width="3800"/><content-rect height="2236" width="3328" x="236" y="120"/><crop-area><point x="135.3065253977068" y="-244"/><point x="3952.297441614658" y="-235.6726044123365"/><point x="3936.990916216952" y="6780.310698751963"/><point x="120" y="6771.9833031643"/><point x="135.3065253977068" y="-244"/></crop-area><partial-xform><m11>-0.00218165983433677</m11><m12>0.9999976201773518</m12><m21>-0.9999976201773518</m21><m22>-0.00218165983433677</m22></partial-xform><dpi horizontal="600" vertical="600"/><color-params colorMode="colorOrGray"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></image><source_file mtime="1698783384" size="2890349"/><file mtime="1699247244" size="12506634"/><zones/><fill-zones/></output-params></page><page id="7"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="colorOrGray"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/><output-params><image blackOnWhite="1" depthPerception="2" despeckleLevel="1"><size height="2476" width="3800"/><content-rect height="2240" width="3328" x="236" y="118"/><crop-area><point x="143.3065253977068" y="-246"/><point x="3953.297458273417" y="-237.6878760311769"/><point x="3937.99093287571" y="6778.295427133123"/><point x="128" y="6769.9833031643"/><point x="143.3065253977068" y="-246"/></crop-area><partial-xform><m11>-0.00218165983433677</m11><m12>0.9999976201773518</m12><m21>-0.9999976201773518</m21><m22>-0.00218165983433677</m22></partial-xform><dpi horizontal="600" vertical="600"/><color-params colorMode="colorOrGray"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></image><source_file mtime="1698783384" size="2890349"/><file mtime="1699247251" size="12520108"/><zones/><fill-zones/></output-params></page></output></filters></project>"#;

    #[test]
    fn it_deserializes() {
        from_str::<ProjectXML>(ORIGINAL_CONTENT).unwrap();
    }

    #[test]
    fn it_validates() {
        let project: ProjectXML = from_str(ORIGINAL_CONTENT).unwrap();
        project.validate().unwrap();
    }

    #[test]
    fn it_serializes() {
        let project: ProjectXML = from_str(ORIGINAL_CONTENT).unwrap();
        to_string(&project).unwrap();
    }

    #[test]
    fn serialized_output_matches_original() {
        let project: ProjectXML = from_str(ORIGINAL_CONTENT).unwrap();
        let output_content = to_string(&project).unwrap();
        assert_eq!(ORIGINAL_CONTENT, output_content);
    }
}
