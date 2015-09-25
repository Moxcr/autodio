//Modifier Metadata

struct EvMeta {
	amt: i32,
	vlt: i32,
	wt: i32
}

struct EmMeta {
	amt: i32,
	vlt: i32,
	wt: i32
}

struct TnMeta {
	amt: i32,
	dir: TransitionDir,
	sp: TransitionSpeed,
}


//Modifier Types

enum EventMod {
	Action(EvMeta),
	Calm(EvMeta),
	Interjection(EvMeta),
	Suspense(EvMeta),
	Clarity(EvMeta),
	Levity(EvMeta),
	Humble(EvMeta),
	Uncertainty(EvMeta),
	Purity(EvMeta),
	Evil(EvMeta),
	Light(EvMeta),
	Dark(EvMeta),
	Expected(EvMeta),
	Anomalous(EvMeta),
	Mysterious(EvMeta),
}

enum EmoteMod {
	Joy(EmMeta),
	Surprise(EmMeta),
	Sadness(EmMeta),
	Fear(EmMeta),
	Anger(EmMeta),
	Craze(EmMeta),
	Angst(EmMeta),
}



enum TransitionMod {
	Hard(TnMeta),
	Soft(TnMeta),
	Staticy(TnMeta),
	Smooth(TnMeta),
	Rocky(TnMeta),
	Glitchy(TnMeta),
}

enum TransitionDir {
	Pre,
	Post,
}

enum TransitionSpeed {
	Instant,
	Fast,
	Medium,
	Slow,
	Lagging,
}