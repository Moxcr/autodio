use modifiers::{EventMod,EmoteMod,TransitionMod};

//Modifier Collection Structs

struct Bucket;

struct Drop {
	events: Vec<EventMod>,
	emotives: Vec<EmoteMod>,
	transitions: TransitionSet,
}

struct TransitionSet {
	preTransition: TransitionMod,
	postTransition: TransitionMod,
}