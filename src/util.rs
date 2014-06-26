pub fn clampf(value: f32, mn: f32, mx: f32) -> f32 {
	if value < mn { mn } else if value > mx { mx } else { value }
}

pub fn lerpf(a: f32, b: f32, t: f32) -> f32 {
	a + (b - a) * t
}

pub fn wrap_radians(radians: f32) -> f32 {
	let two_pi: f32 = Float::two_pi();
	let mut r = radians;
	while r < 0_f32 {
		r += two_pi;
	}
	while r > two_pi {
		r -= two_pi;
	}
	return r;
}