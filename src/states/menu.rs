extern crate amethyst;
use amethyst::{
	prelude::*,
	ecs::Entity,
	ui::{
		UiEvent,
		UiEventType,
		UiFinder
	}
};

pub struct MainMenuState;

impl SimpleState for MainMenuState{}