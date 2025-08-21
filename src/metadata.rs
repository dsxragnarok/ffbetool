use crate::{Result, cgs, imageops};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationJson {
    pub unit_id: u32,
    pub anim_name: String,
    pub frame_delays: Vec<u32>,
    pub frame_rect: RectJson,
    pub image_width: u32,
    pub image_height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RectJson {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl AnimationJson {
    pub fn from_frames(
        unit_id: u32,
        anim_name: String,
        frames: &[cgs::CompositeFrame],
        frame_rect: &imageops::Rect,
        spritesheet_width: u32,
        spritesheet_height: u32,
    ) -> Self {
        let frame_delays = frames.iter().map(|f| f.delay).collect();

        Self {
            unit_id,
            anim_name,
            frame_delays,
            frame_rect: RectJson {
                x: frame_rect.x,
                y: frame_rect.y,
                width: frame_rect.width,
                height: frame_rect.height,
            },
            image_width: spritesheet_width,
            image_height: spritesheet_height,
        }
    }
}

pub fn save_animation_json(animation_json: &AnimationJson, output_path: &str) -> Result<()> {
    let json_content = serde_json::to_string_pretty(animation_json)?;
    std::fs::write(output_path, json_content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imageops::Rect;

    #[test]
    fn test_animation_json_from_frames() {
        let frames = vec![
            cgs::CompositeFrame {
                frame_idx: 0,
                image: image::RgbaImage::new(50, 50),
                rect: Rect {
                    x: 10,
                    y: 20,
                    width: 50,
                    height: 50,
                },
                delay: 6,
            },
            cgs::CompositeFrame {
                frame_idx: 1,
                image: image::RgbaImage::new(50, 50),
                rect: Rect {
                    x: 10,
                    y: 20,
                    width: 50,
                    height: 50,
                },
                delay: 5,
            },
            cgs::CompositeFrame {
                frame_idx: 2,
                image: image::RgbaImage::new(50, 50),
                rect: Rect {
                    x: 10,
                    y: 20,
                    width: 50,
                    height: 50,
                },
                delay: 4,
            },
        ];

        let frame_rect = Rect {
            x: 902,
            y: 873,
            width: 188,
            height: 168,
        };

        let animation_json = AnimationJson::from_frames(
            401012417,
            "atk".to_string(),
            &frames,
            &frame_rect,
            752,
            1344,
        );

        assert_eq!(animation_json.unit_id, 401012417);
        assert_eq!(animation_json.anim_name, "atk");
        assert_eq!(animation_json.frame_delays, vec![6, 5, 4]);
        assert_eq!(animation_json.frame_rect.x, 902);
        assert_eq!(animation_json.frame_rect.y, 873);
        assert_eq!(animation_json.frame_rect.width, 188);
        assert_eq!(animation_json.frame_rect.height, 168);
        assert_eq!(animation_json.image_width, 752);
        assert_eq!(animation_json.image_height, 1344);
    }

    #[test]
    fn test_json_serialization() {
        let animation_json = AnimationJson {
            unit_id: 401012417,
            anim_name: "limit_atk".to_string(),
            frame_delays: vec![6, 5, 5, 5],
            frame_rect: RectJson {
                x: 902,
                y: 873,
                width: 188,
                height: 168,
            },
            image_width: 752,
            image_height: 1344,
        };

        let json_result = serde_json::to_string_pretty(&animation_json);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();

        // Parse the JSON back to validate structure
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Validate all required fields exist and have correct values
        assert_eq!(parsed["unitId"], 401012417);
        assert_eq!(parsed["animName"], "limit_atk");
        assert_eq!(parsed["frameDelays"], serde_json::json!([6, 5, 5, 5]));
        assert_eq!(parsed["frameRect"]["x"], 902);
        assert_eq!(parsed["frameRect"]["y"], 873);
        assert_eq!(parsed["frameRect"]["width"], 188);
        assert_eq!(parsed["frameRect"]["height"], 168);
        assert_eq!(parsed["imageWidth"], 752);
        assert_eq!(parsed["imageHeight"], 1344);

        // Ensure no extra fields at root level
        let expected_keys = [
            "unitId",
            "animName",
            "frameDelays",
            "frameRect",
            "imageWidth",
            "imageHeight",
        ];
        assert_eq!(parsed.as_object().unwrap().len(), expected_keys.len());
        for key in expected_keys {
            assert!(parsed.as_object().unwrap().contains_key(key));
        }

        // Ensure frameRect has exactly the expected fields
        let frame_rect_obj = parsed["frameRect"].as_object().unwrap();
        let expected_rect_keys = ["x", "y", "width", "height"];

        assert_eq!(frame_rect_obj.len(), expected_rect_keys.len());
        for key in expected_rect_keys {
            assert!(frame_rect_obj.contains_key(key));
        }

        // Validate data types
        assert!(parsed["unitId"].is_u64());
        assert!(parsed["animName"].is_string());
        assert!(parsed["frameDelays"].is_array());
        assert!(parsed["frameRect"].is_object());
        assert!(parsed["imageWidth"].is_u64());
        assert!(parsed["imageHeight"].is_u64());

        // Validate frameDelays array contains only numbers
        let delays = parsed["frameDelays"].as_array().unwrap();
        for delay in delays {
            assert!(delay.is_u64());
        }
    }
}
