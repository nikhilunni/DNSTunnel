#include <server.h>
#include <boost/bind.hpp>
#include <boost/shared_ptr.hpp>
#include <boost/asio.hpp>
#include <iostream>

using namespace boost;

void Server::run() {
  udp_socket.async_receive_from(asio::buffer(recv_buffer), remote_endpoint,
				bind(&Server::handle_request,
				     this,
				     asio::placeholders::error,
				     asio::placeholders::bytes_transferred
				     )
				);
}

void Server::handle_request(const system::error_code& error, std::size_t bytes) {
  if (!error) {
    shared_ptr<std::string> message(new std::string(recv_buffer.begin(), recv_buffer.begin() + bytes));
    std::cout << *message << std::endl;
    run();
  }
}
