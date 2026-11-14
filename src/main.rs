use pnet::datalink::{self, NetworkInterface};
use pnet::packet::Packet;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

fn main() {

    let interfaces: Vec<NetworkInterface> = datalink::interfaces();

    let interfaz_elegida = interfaces

        .into_iter()

        .find(|iface| iface.is_up() && !iface.is_loopback() && !iface.ips.is_empty());

    if let Some(interfaz) = interfaz_elegida {
        println!(
            "La interfaz seleccionada por default ha sido, {}",
            interfaz.name
        );


        let canal = datalink::channel(&interfaz, Default::default());

        match canal {
            Ok(canal) => {
                println!("Canal abierto");

                match canal {
                    datalink::Channel::Ethernet(_tx, mut rx) => {
                        println!(
                            "El receptor ha sido encerrado contra su voluntad para que lo manipules felizmente, ¿Contento monstruo?"
                        );
                        loop {
                            match rx.next() {
                                Ok(paquete) => {
                                    procesar_paquetes(paquete);
                                }

                                Err(e) => {
                                    println!("No hay paquetes ¿Por que? yo que se {}", e)
                                }
                            }
                        }
                    }

                    _ => {
                        println!(
                            "estate quieto que me quemas la tarjeta red, el tx esta enterrado bajo tierra"
                        );
                    }
                }
            }
            Err(error) => {
                println!(
                    "El canal no se pudo abrir, faltan permisos o algo yo q se, llama al programador y que tlabaje {}",
                    error
                );
            }
        }
    } else {
        println!("No se ha encontrado la intefaz predeterminada, error");
        return;
    }
}

fn procesar_paquetes(paquete: &[u8]) {
    if let Some(paquete_ethernet) = EthernetPacket::new(paquete) {
        let mac_origen = paquete_ethernet.get_source();
        let mac_destino = paquete_ethernet.get_destination();
        let tipo_protocolo = paquete_ethernet.get_ethertype();

        if tipo_protocolo == EtherTypes::Ipv4 {

            al siguiente
            */
            if let Some(paquete_ipv4) = Ipv4Packet::new(paquete_ethernet.payload()) {
                let ip_origen = paquete_ipv4.get_source();
                let ip_destino = paquete_ipv4.get_destination();
                println!(
                    "---------------------------------------------------------------------------------------"
                );
                println!("Ip origen {} | Ip destino {}", ip_origen, ip_destino);

                let protocolo_transporte = paquete_ipv4.get_next_level_protocol();

                if protocolo_transporte == IpNextHeaderProtocols::Udp {
                    if let Some(paquete_tcp) = TcpPacket::new(paquete_ipv4.payload()) {
                        let puerto_origen = paquete_tcp.get_source();
                        let puerto_destino = paquete_tcp.get_destination();


                        if puerto_origen != 80 && puerto_origen != 443 && puerto_destino != 80 && puerto_destino != 443 {
                            return;
                        }
                        println!(
                            "TCP Puerto de origen {} | Puerto de destino {}",
                            puerto_origen, puerto_destino
                        );

                        let payload_crudo = paquete_tcp.payload();

                        if !payload_crudo.is_empty() {
                            let texto_traducido = String::from_utf8_lossy(payload_crudo);

                            println!(" Datos interceptados");
                            println!("{}", texto_traducido) 
                        }
                    }
                } else if protocolo_transporte == IpNextHeaderProtocols::Udp {
                    if let Some(paquete_udp) = UdpPacket::new(paquete_ipv4.payload()) {
                        let puerto_origen = paquete_udp.get_source();
                        let puerto_destino = paquete_udp.get_destination();

                        println!(
                            "Udp Puerto de origen {} | Puerto de destino {}",
                            puerto_origen, puerto_destino
                        );
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
        println!(
            "---------------------------------------------------------------------------------------"
        );
    }
}
