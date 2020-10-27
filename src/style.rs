use iced::{container, Background, Color};

pub struct Connections;
impl container::StyleSheet for Connections {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.09, 0.24, 0.29))),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}
