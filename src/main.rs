use std::net::TcpListener;
use std::result;
use std::io::Write;

type Result<T>=result::Result<T,()>;

fn main()->Result<()> {
    let address="127.0.0.1:6969";
   let _listener= TcpListener::bind(address).map_err(|err|{
    eprint!("ERROR:Could not bind {address} : {err}")
   })?;
   print!("Listen to {address}");
   for stream in _listener.incoming(){
    match stream{
        Ok(mut steam)=>{

          let _=  writeln!(steam,"Hallo Mein Führer ;) ").map_err(|err|{
                eprintln!("ERROR: could not write msg to use : {err}, ")

            });
        
        }
            Err(err)=>{ 
            eprintln!("ERROR: could not accept connection: {err}, ")
        }
    }

   }
Ok(())
}
