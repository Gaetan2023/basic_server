use std::io::{Read, Write};
use std::net::{TcpListener,TcpStream};
use std::{fs, prelude::*};
use salutation::GroupeTaches;

fn main(){
    let ecouter =TcpListener::bind("127.0.0.1:7878").unwrap();
    let groupe = GroupeTaches::new(4);

    for flux in ecouter.incoming(){
        let flux = flux.unwrap();

        groupe.execute(||{
            gestion_connexion(flux);

        });
    }
}

fn gestion_connexion(mut flux:TcpStream){
    let mut tampon =[0;1024];
    flux.read(& mut tampon).unwrap();
    let get= b"GET / HTTP/1.1\r\n";

    let (ligne_status,nom_fichier)=if tampon.starts_with(get){
        ("HTTP/1.1 200 OK","hello.html")
    } else{
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };
    let contenu =fs::read_to_string(nom_fichier).unwrap();
    let reponse= format!("{ligne_status}\r\nContent-lengh:{}\r\n\r\n{}",contenu.len(),contenu);
    
    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
} 