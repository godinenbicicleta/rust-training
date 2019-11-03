pub mod moduloeqs{
	#[derive(Debug,PartialEq)]
	pub enum GradoEcuacion {
	   PRIMERORDEN,
	   SEGUNDOORDEN,
	   CERO,
	}

	#[derive(Debug)]
	pub struct NumeroComplejo {
	  real: f64,
	  imaginario: f64,
	}

	#[derive(Debug)]
	pub enum RaizSegundoGrado {
		 Reales(f64,f64),
		 Complejas(NumeroComplejo,NumeroComplejo),
	}

	#[derive(Debug)]
	pub enum Raiz {
	  PrimerGrado(f64),
	  SegundoGrado(RaizSegundoGrado),
	}

	pub fn grado_ecuacion(a: f64, b:f64, c:f64) -> GradoEcuacion {
	  if a == 0.0 {
		if  b == 0.0 {
		   GradoEcuacion::CERO
		}  else {
		   GradoEcuacion::PRIMERORDEN
		}
	 } else {
		GradoEcuacion::SEGUNDOORDEN
	 }
	}

	fn discriminante(a: f64, b:f64, c:f64) -> f64 {
	  b*b - 4.0*a*c
	}

	fn son_raices_reales(a: f64, b:f64, c:f64) -> Result<bool,()> {
	   let grado= grado_ecuacion(a,b,c);
	   match grado {
		 GradoEcuacion::SEGUNDOORDEN => Ok(discriminante(a,b,c) >= 0.0f64) ,
		 _ => Err(()),
	   }
	}

	fn son_raices_complejas(a: f64, b:f64, c:f64) -> Result<bool,()>  {
	   let grado= grado_ecuacion(a,b,c);
	   match grado {
		 GradoEcuacion::SEGUNDOORDEN => Ok(discriminante(a,b,c) < 0.0f64) ,
		 _ => Err(()),
	   }
	}

	fn ecuacion_1_grado(a: f64, b:f64 )-> Result<f64,()>
	{
	   if a!=0.0 {
		 Ok(-b/a)
	   } else {
		 Err(())
	   }
	}
	fn ecuacion_2_grado(a: f64, b:f64, c:f64 )->
					 Result<Raiz,String> {
	  /*let raiz_grado_2_real = Raiz::SegundoGrado(
		   RaizSegundoGrado::Reales(-3f64,-4f64));
		   */
	  let grado = grado_ecuacion(a,b,c);
	  match grado {
		  GradoEcuacion::SEGUNDOORDEN => {
			 if son_raices_reales(a,b,c).unwrap() {
				 let d= discriminante(a,b,c);
				 let raiz_grado_2_real = Raiz::SegundoGrado(
					  RaizSegundoGrado::Reales( (-b + d.sqrt())/2.0*a,
											   (-b - d.sqrt())/2.0*a));
				 Ok(raiz_grado_2_real)
			 } else {
				 let d= discriminante(a,b,c).abs().sqrt();
				 let complejo1 = NumeroComplejo{real:-b/2.0*a,imaginario:d/2.0*a};
				 let complejo2 = NumeroComplejo{real:-b/2.0*a,imaginario:-d/2.0*a};
				 let raiz_grado_2_compleja = Raiz::SegundoGrado(
					  RaizSegundoGrado::Complejas(complejo1,complejo2) );
				 Ok(raiz_grado_2_compleja)
			 }
		  } ,
		  _ => Err(String::from("No es ecuacion de segundo grado")) ,
	  }

	}

}

#[test]
fn test_enumeraciones() {
  let raiz_grado_1 = Raiz::PrimerGrado(1f64);
  let raiz_grado_2_real = Raiz::SegundoGrado(
       RaizSegundoGrado::Reales(-3f64,-4f64));
  let raiz_grado_2_compleja = Raiz::SegundoGrado(
       RaizSegundoGrado::Complejas(NumeroComplejo{real:1f64,imaginario:1f64},
                              NumeroComplejo{real:1f64,imaginario:-1f64}));
}

fn test_grado_ecuacion() {
   assert_eq!(grado_ecuacion(0f64,0f64,1f64) , GradoEcuacion::CERO);
   assert_eq!(grado_ecuacion(0f64,1f64,1f64) , GradoEcuacion::PRIMERORDEN);
   assert_eq!(grado_ecuacion(1f64,1f64,1f64) , GradoEcuacion::SEGUNDOORDEN);
}

fn test_discriminante_ecuacion() {
	assert!(son_raices_reales(1f64,7f64,12f64).unwrap());
	assert!(son_raices_reales(0f64,7f64,12f64).is_err());
	assert!(son_raices_complejas(1f64,1f64,1f64).unwrap());
	assert!(son_raices_complejas(0f64,7f64,12f64).is_err());
}

fn test_ecuacion_primer_grado() {
   assert_eq!(grado_ecuacion(0f64,1f64,1f64) , GradoEcuacion::PRIMERORDEN);
   assert_eq!(ecuacion_1_grado(1f64,2f64).unwrap() , -2f64);
   assert!(ecuacion_1_grado(0f64,2f64).is_err());
}

fn test_ecuacion_segundo_grado_solucionreal() {
	assert_eq!(grado_ecuacion(1f64,7f64,12f64) , GradoEcuacion::SEGUNDOORDEN);
	let solucion01 =
	ecuacion_2_grado(1f64,7f64,12f64).unwrap();
	match solucion01 {
		Raiz::SegundoGrado(raices) => {

			match raices  {
				RaizSegundoGrado::Reales(r1,r2) => println!("raices {:?}  {:?}",r1,r2  ) ,
				_ => {println!("raices {:?}",raices  );} ,
			}
		},
		_ => println!("excepcion"),
	}


	assert!(false);
}

fn test_ecuacion_segundo_grado_solucioncompleja() {
	assert_eq!(grado_ecuacion(1f64,1f64,1f64) , GradoEcuacion::SEGUNDOORDEN);
	let solucion01 =
	ecuacion_2_grado(1f64,1f64,1f64).unwrap();
	match solucion01 {
		Raiz::SegundoGrado(raices) => {

			match raices  {
				RaizSegundoGrado::Complejas(c1,c2) =>
						println!("raices {:?} + j{} ;  {:?} + j{:?}",
						 c1.real,c1.imaginario,
						 c2.real,c2.imaginario,  ) ,
				_ => {println!("raices {:?}",raices  );} ,
			}
		},
		_ => println!("excepcion"),
	}


	assert!(false);
}
