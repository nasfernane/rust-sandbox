fn main() {
  // pour les variables locales (let), l'annotation de type est requise (et souhaitable) seulement quand le type ne peut être correctement inféré exemple avec parse
  // let guess = "42".parse().expect("Not a number"); // error[E0284]: type annotations needed

  // quand le type est ambigu (ne peut pas être inféré de manière fiable), l'annotation de type est nécessaire
  let _guess: u32 = "42".parse().expect("Not a number"); // le préfixe _ signifie que la non utilisation de la variable est intentionnelle, pas de warning remonté par le compilateur

  // SCALAIRES
  // un type scalaire représente une seule valeur. Les types scalaires en rust sont: integers, floating-point numbers, Booleans, et characters

  // Integers
  // i = signed, u => unsigned;
  // default integer value is u32
  // let _value1: i8 // 8bit signed de -128 à +127
  // let _value2: u8 // 8 bit unsigned de 0 à 255

  // isize et usize dépendent de l'architecture de l'ordinateur qui fait tourner le programme. 64bits sur une architecture 64-bit ou 32 bit sur une archi 32-bit

  // Floating numbers; la valeur par défaut est f64 car plus ou moins aussi rapide mais avec davantage de précision
  // let x = 2.0; // f64
  // let y: f32 = 3.0; // f32

  // boolean
  // let t = true;
  // let f: bool = false; // annotation explicite


  // character - valeur scalaire unicode; il se déclare avec des guillemets simples
  let c = 'z';
  let z: char = 'Z'; // annotation explicite
  let heart_eyed_cat = '😻';
  let oupss = "z" // type string pas char, à cause des guillements doubles


}
