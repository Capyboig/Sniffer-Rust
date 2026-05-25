use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

fn main() {
    //Let: Declaramos la variable inmutable por que no metemos &mut
        //interfaces: nombre de la variable, gilipollas que crees q era.
        //Vec<NetworkInterface> es como un map, un array dinamico que meteremos las interfaces
        /*datalink::interfaces(). Los 2 puntillos esos son pa llamar a una funcion dentro del
          pive que va a traer las interfaces red, sin importar si es linux o windows
         */
    let interfaces: Vec<NetworkInterface> = datalink::interfaces();

    let interfaz_elegida= interfaces
                //into_iter: esta shit es pa que el vector sea iterable, pa que vaya avanzando 1 a 1
        .into_iter()
        //filter: esta shit es literalmente un if, cada dato que entra si pasa esto va pa lante
                //|iface|: basicamente, cada una lo llama, lo mete en un paquetito pa los filtros y que sea individual        
                    //iface.is_up: Sabes ingles subnormal te dije que es como un if, lee bien?
                        //iface.is_loopback: pa evitar el localhost y el 127.0.0.1
                            //iface.name ==: Broder si nombre de variable = al eth0 tira pa lante, no hay mas misterio
        .find(|iface| iface.is_up() && !iface.is_loopback() && iface.name == "enp0s3");
            //collect: pa que lo que sobreviva lo meta en el vector de arriba
        

        if let Some(interfaz) = interfaz_elegida {
            println!("La interfaz seleccionada por default ha sido, {}", interfaz.name);
                
                //canal: La porta q por detras tiene los datos, tu lo abres y ahi esta to la shit pero te pide 2 mierdas q t explico adelante
                    //datalink::channel: El export pa la mierda del channel, si quieres saber que hace buscalo en la web que son 2 tonterias
                        //&interfaz y default: esas 2 shits es una pides prestada la interfaz y la otra la config que ponemos default por que no sabemos aun una mierda, lo dejamos ahi predeterminao
            let canal = datalink::channel(&interfaz, Default::default());
                //match: por lo que comprendi es la cajita pa obligarte a atender a las 2 posibilidades que te trae, el bien o el error.
            match canal {
                    //El canal este es un chivato que te dice lo que hay por ahi, dentro tu decides si recivir o transmitir
                Ok(canal) => {
                    println!("Canal abierto");

                    //match: esto es el segundo match pa separar a machetazos al pive que habla del que usa las manos
                    match canal {
                        //otro import, me da pereza explicarlo pero si no lo hago quemo algo;
                            //el _tx es el receptor y lo colocamos asi no se si pa identificarlo pa separarlo y ya o paq
                                //el rx lo dejamos mutable porq lo cambiaremos o algo supongo
                        datalink::Channel::Ethernet(_tx,mut rx ) => {
                            println!("El receptor ha sido encerrado contra su voluntad para que lo manipules felizmente, ¿Contento monstruo?");
                             loop {
                                match rx.next() {
                                Ok(paquete) => {
                                        //El some es del paquete que entra, nos dice que puede llegar algo bien o mal, ya te la sabes
                                            //si el paquete es un ethernetPackage osea uno valido, crea un paquete enterito con las especificaciones ahi todas juntas
                                    if let Some(paquete_ethernet) = EthernetPacket::new(paquete) {
                                            //Esto son solo variables para separar las cosas del paquete y que no sean solo numeros
                                        let mac_origen = paquete_ethernet.get_source();
                                        let mac_destino = paquete_ethernet.get_destination();
                                        let tipo_protocolo = paquete_ethernet.get_ethertype();
                                        
                                        if tipo_protocolo == EtherTypes::Ipv4 {
                                                //Todo de aqui para abajo es intuitivo si te has leido todo el proceso, mucho texto me da pereza escribir
                                                    //el payload te lo perdono, aqui la explicacion.
                                                        /* 
                                                        Payload: basicamente, si esto es un mega paquete de todo, pillamos la etiqueta de lo que hay en la caja
                                                        basicamente, lo que necesite y dentro el programa ve otra caja y en vez de asumir la deja ahi, las 
                                                        cajas como tal no estan creadas siempre, las fuimos creando y organizando, usando el payload nos
                                                        aseguramos que cada interferencia que captemos de un dato en concreto lo captemos integro sin molestar
                                                        al siguiente
                                                        */
                                            if let Some(paquete_ipv4) = Ipv4Packet::new(paquete_ethernet.payload()) {

                                                let ip_origen = paquete_ipv4.get_source();
                                                let ip_destino = paquete_ipv4.get_destination();
                                                println!("---------------------------------------------------------------------------------------"); 
                                                println!("Ip origen {} | Ip destino {}", ip_origen, ip_destino);

                                                let protocolo_transporte = paquete_ipv4.get_next_level_protocol();

                                                if protocolo_transporte == IpNextHeaderProtocols::Udp {

                                                    if let Some(paquete_tcp) = TcpPacket::new(paquete_ipv4.payload()) {
                                                        let puerto_origen = paquete_tcp.get_source();
                                                        let puerto_destino = paquete_tcp.get_destination();

                                                        println!("TCP Puerto de origen {} | Puerto de destino {}", puerto_origen, puerto_destino); 
                                                    }
                                                } else if protocolo_transporte == IpNextHeaderProtocols::Udp {

                                                    if let Some(paquete_udp) = UdpPacket::new(paquete_ipv4.payload()) {
                                                        let puerto_origen = paquete_udp.get_source();
                                                        let puerto_destino = paquete_udp.get_destination();


                                                        println!("Udp Puerto de origen {} | Puerto de destino {}", puerto_origen, puerto_destino);
                                                    }
                                                }
                                            }
                                        }
                                        println!(
                                            "Tamaño del paquete: {} | Origen {} | Destino {} | Protocolo {}",
                                            paquete.len(),
                                            mac_origen,
                                            mac_destino,
                                            tipo_protocolo

                                        );
                                        println!("---------------------------------------------------------------------------------------");
                                    }
                                } 

                                Err(e) => {
                                    println!("No hay paquetes ¿Por que? yo que se {}", e)
                                } 
                            } 
                            }
                            
                        },

                        _ => {
                            println!("estate quieto que me quemas la tarjeta red, el tx esta enterrado bajo tierra");
                        }
                    }
                }
                    //Broski si has llegado hasta aqui y te paras a leer que es el error desinstala el IDE y vuelve a pseint
                Err(error) => {
                    println!("El canal no se pudo abrir, faltan permisos o algo yo q se, llama al programador y que tlabaje {}", error);
                }
            }
        } else {
            println!("No se ha encontrado la intefaz predeterminada, error");
            return;
        } 


}

