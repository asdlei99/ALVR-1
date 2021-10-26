use super::{
    tabs::{ConnectionEvent, ConnectionPanel},
    theme::{ContainerStyle, ACCENT, BACKGROUND_SECONDARY, FOREGROUND},
};
use alvr_common::ServerEvent;
use iced::{
    button, image, Alignment, Button, Column, Container, Element, Image, Length, Row, Space, Text,
};

pub enum TabLabelStyle {
    Normal,
    Selected,
}

impl button::StyleSheet for TabLabelStyle {
    fn active(&self) -> button::Style {
        let normal = button::Style {
            background: BACKGROUND_SECONDARY.into(),
            border_radius: 10.0,
            text_color: FOREGROUND,
            ..Default::default()
        };

        match self {
            TabLabelStyle::Normal => normal,
            TabLabelStyle::Selected => button::Style {
                background: ACCENT.into(),
                ..normal
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum DashboardEvent {
    ServerEvent(ServerEvent),
    TabClick(usize),
    LanguageClick,
    ConnectionEvent(ConnectionEvent),
}

pub struct TabState {
    icon: (), // todo
    display_name: String,
    state: button::State,
}

impl Default for TabState {
    fn default() -> Self {
        Self {
            icon: (),
            display_name: "".into(),
            state: Default::default(),
        }
    }
}

pub struct Dashboard {
    selected_tab: usize,
    tab_states: Vec<TabState>,
    language_state: TabState,
    connection_panel: ConnectionPanel,
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            selected_tab: 0,
            tab_states: vec![
                TabState {
                    display_name: "Connection".into(),
                    ..Default::default()
                },
                TabState {
                    display_name: "Statistics".into(),
                    ..Default::default()
                },
                TabState {
                    display_name: "Settings".into(),
                    ..Default::default()
                },
                TabState {
                    display_name: "Installation".into(),
                    ..Default::default()
                },
                TabState {
                    display_name: "Logs".into(),
                    ..Default::default()
                },
                TabState {
                    display_name: "About".into(),
                    ..Default::default()
                },
            ],
            connection_panel: ConnectionPanel::default(),
            language_state: TabState {
                display_name: "Language".into(),
                ..Default::default()
            },
        }
    }
}

impl Dashboard {
    pub fn update(
        &mut self,
        event: DashboardEvent,
        request_handler: &mut dyn FnMut(String) -> String,
    ) {
        match event {
            DashboardEvent::ServerEvent(_) => (),
            DashboardEvent::TabClick(tab) => self.selected_tab = tab,
            DashboardEvent::LanguageClick => (),
            DashboardEvent::ConnectionEvent(event) => {
                self.connection_panel.update(event, request_handler)
            }
        }
    }

    pub fn view(&mut self) -> Element<DashboardEvent> {
        let mut sidebar_children = vec![Image::new(image::Handle::from_memory(
            include_bytes!("../../../resources/images/favicon.png").to_vec(),
        ))
        .into()];

        // work around "self.tab_states cannot be borrowed both mutably and immutably"
        let mut selected_tab_display_name = "".into();

        for (index, state) in self.tab_states.iter_mut().enumerate() {
            if index == self.selected_tab {
                selected_tab_display_name = state.display_name.clone();
            }

            sidebar_children.push(
                Button::new(
                    &mut state.state,
                    Row::with_children(vec![
                        Image::new(image::Handle::from_memory(
                            include_bytes!("../../../resources/images/favicon.png").to_vec(),
                        ))
                        .into(),
                        Text::new(&state.display_name).into(),
                    ])
                    .spacing(5),
                )
                .style(if self.selected_tab == index {
                    TabLabelStyle::Selected
                } else {
                    TabLabelStyle::Normal
                })
                .on_press(DashboardEvent::TabClick(index))
                .into(),
            );
        }

        sidebar_children.push(Space::with_height(Length::Fill).into());
        sidebar_children.push(
            Button::new(
                &mut self.language_state.state,
                Text::new(&self.language_state.display_name),
            )
            .style(TabLabelStyle::Normal)
            .on_press(DashboardEvent::LanguageClick)
            .into(),
        );

        let content_panel = match self.selected_tab {
            0 => self
                .connection_panel
                .view()
                .map(DashboardEvent::ConnectionEvent),
            _ => Text::new("unimplemented").into(),
        };

        Container::new(Row::with_children(vec![
            Column::with_children(sidebar_children)
                .padding(5)
                .spacing(5)
                .align_items(Alignment::Fill)
                .into(),
            Column::with_children(vec![
                Text::new(selected_tab_display_name).size(30).into(),
                content_panel,
            ])
            .width(Length::Fill)
            .into(),
        ]))
        .style(ContainerStyle)
        .into()
    }
}