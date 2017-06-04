#include <udp_server.h>
#include <boost/bind.hpp>
#include <boost/shared_ptr.hpp>
#include <boost/asio.hpp>
#include <iostream>
#include <iomanip>

void dns_tunnel::udp::UDPServer::run() {
  socket.async_receive_from(boost::asio::buffer(recv_buffer),
                            endpoint,
                            bind(&dns_tunnel::udp::UDPServer::handle_request,
                                 this,
                                 boost::asio::placeholders::error,
                                 boost::asio::placeholders::bytes_transferred
                                 )
                            );

}

void dns_tunnel::udp::UDPServer::handle_request(const boost::system::error_code& error, std::size_t bytes) {
  if (!error) {
    auto message(new std::string(recv_buffer.begin(), recv_buffer.begin() + bytes));
    for(std::string::iterator it = message->begin(); it != message->end(); ++it) {
      std::cout << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(*it);
    }
    std::cout << std::endl;
    run();
  }
}
