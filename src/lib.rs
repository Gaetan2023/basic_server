use std::mem::ManuallyDrop;
use std::thread;
use std::sync::{mpsc,Arc,Mutex};



pub struct GroupeTaches{
    operateurs:Vec<Operateur>,
    envoi:mpsc::Sender<Message>
}
impl GroupeTaches {
    ///creer un groupe de taches 
    /// le nombre de taches sera la valeur passer a la fonction new
    /// # Panique
    /// le servreur paniquer si le nombre est egale a zero
    pub fn new(size:usize)-> GroupeTaches{
        assert!(size>0);//verification du nombre

        //creaction du vecteur des operateurs avec une capacite size
        let mut operateurs =Vec::with_capacity(size);

        //creer un canal
        let (envoi,recevoir) =mpsc::channel();
        //partage des resources
        let reception =Arc::new(Mutex::new(recevoir));
        
        for id in 0..size{
            operateurs.push(Operateur::new(id,Arc::clone(&reception)));
        }
        GroupeTaches{operateurs,envoi}
    }

    /// la fonction execute devra recevoir une closure comme params(FnOnce) 
    /// qui poura etre envoyer entre les taches(Send)
    /// et aura une duree de vie static
    pub fn execute<F>(&self,f:F) 
    where F: FnOnce()+Send+'static
               {
                     let mission =Message::NouvelleMission(Box::new(f));
                     self.envoi.send(mission).unwrap();
               }
               
}

impl Drop for GroupeTaches{
 fn drop(&mut self) {
     println!("Envoi le message d'extintion a toutes les taches.");
     for _ in &self.operateurs  {
        self.envoi.send(Message::Extinction).unwrap();
         
     }
     println!("stop de tout les operateurs.");
     for operateur in &mut self.operateurs  {
         println!("Arret de l'operateur {}",operateur.id);
         if let Some(tache) = operateur.tache.take() {
             tache.join().unwrap();
         }
     }
 }
}
struct Operateur{
    id:usize,
    tache:Option<thread::JoinHandle<()>>
}
impl Operateur {
    fn new(id:usize,reception:Arc<Mutex<mpsc::Receiver<Message>>>)->Operateur{

        let tache=thread::spawn(move||loop {
            let message = reception.lock().unwrap().recv()  {
               
               match message {
               Ok( Message::NouvelleMission(mission) )=>{
                    println!("L'operation {} a obtenu une mission et l'execute ",id);
                    mission();
                },
                Message::Extinction =>{
                    println!("L'operation {} a recu l'instruction d'arret.",id);
                    break;
                }
                   
               }
              
                
            }
        });
        Operateur{id,Some(tache),}
    }
    
}
//transformation de mission en boite qui va recevoir la closure
type  Mission =Box<dyn FnOnce()+Send+'static>;
enum Message {
    NouvelleMission(Mission),
    Extinction,
}




