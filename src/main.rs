use  std::marker::PhantomData;

pub  trait Envoi{}
impl <T:  Send> Envoi for T {}

pub trait Synchro  {}

impl<T: Sync > Synchro for T {}

pub struct Liste<T ,L > {
    donne: Vec<T>,
        _marqueur: PhantomData<L>,

}

impl <T , L: 'static> Liste<T, L> {
    pub fn nouveau() -> Self {
        Liste{
        donne: Vec::new(),
        _marqueur: PhantomData,
        }
    }
    pub fn mettre(&mut self, objet : T ){
        self.donne.push(objet);
    }

    pub fn  obtenir(&self) -> &[T]{
        &self.donne
    }
}

fn main(){

    let mut liste : Liste<i32, &'static str> = Liste::nouveau();
    let  mut liste2 : Liste<String, &'static str> = Liste::nouveau();

liste2.mettre("comment  bien comprendre  les  parameters phantom ".to_string());
    liste2.mettre("les  paramete phantom sont  tres importent".to_string());

    liste.mettre(2);
    liste.mettre(5);
    liste.mettre(789);

    println!("les elements de la  liste  sont {:?}", liste.obtenir()); 
    println!("les  element  de  la deuxieme liste {:?}", liste2.obtenir()); 

}