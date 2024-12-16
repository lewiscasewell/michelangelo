use rand::{thread_rng, Rng};

const QUOTES: &[&str] = &["The greater danger for most of us lies not in setting our aim too high and falling short, but in setting our aim too low, and achieving our mark.",
"I saw the angel in the marble and carved until I set him free.",
"Lord, grant that I may always desire more than I can accomplish.",
"Trifles make perfection, and perfection is no trifle.",
"I am still learning.",
"Death and love are the two wings that bear the good man to heaven.",
"It is necessary to keep one’s compass in one’s eyes and not in the hand, for the hands execute, but the eye judges.",
"What spirit is so empty and blind, that it cannot recognize the fact that the foot is more noble than the shoe, and skin more beautiful than the garment with which it is clothed?",
"Many believe – and I believe – that I have been designated for this work by God. In spite of my old age, I do not want to give it up; I work out of love for God, and I put all my hope in Him.",
"The true work of art is but a shadow of the divine perfection.",
"My soul can find no staircase to Heaven unless it be through Earth's loveliness.",
"A beautiful thing never gives so much pain as does failing to hear and see it.",
"There is no greater harm than that of time wasted.",
"The marble not yet carved can hold the form of every thought the greatest artist has.",
"Faith in oneself is the best and safest course.",
"Carving is easy, you just go down to the skin and stop.",
"The more the marble wastes, the more the statue grows.",
"I cannot live under pressures from patrons, let alone paint.",
"Every block of stone has a statue inside it and it is the task of the sculptor to discover it.",
"If people knew how hard I worked to get my mastery, it wouldn't seem so wonderful at all.",
"Genius is eternal patience.",
"The greatest artist has no conception which a single block of white marble does not potentially contain within its mass, but only a hand obedient to the mind can penetrate to this image.",
"A man paints with his brains and not with his hands.",
"Every beauty which is seen here by persons of perception resembles more than anything else that celestial source from which we all are come.",
"It is well with me only when I have a chisel in my hand.",
"Even if you are divine, you don’t disdain male consorts.",
"The best of artists has no conception that the marble alone does not contain within itself.",
"Good painting is the kind that looks like sculpture.",
"What do you despise? By this you are truly known.",
"I live and love in God’s peculiar light.",
"I live in sin, to kill myself I live; no longer my life my own, but sin’s; my good is given to me by heaven, my evil by myself, by my free will, of which I am deprived.",
"I have never felt salvation in nature. I love cities above all.",
"If in my youth I had realized that the sustaining splendor of beauty with which I was in love would one day flood back into my heart, there to ignite a flame that would torture me without end, how gladly would I have put out the light in my eyes.",
"If we have been pleased with life, we should not be displeased with death, since it comes from the hand of the same master.",
"The promises of this world are, for the most part, vain phantoms; and to confide in one’s self, and become something of worth and value is the best and safest course.",
"I am a poor man and of little worth, who is laboring in that art that God has given me in order to extend my life as long as possible.",
"From such a gentle thing, from such a fountain of all delight, my every pain is born."];

pub fn get_random_quote() -> &'static str {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..QUOTES.len());
    QUOTES[index]
}
