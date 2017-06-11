#ifndef SERVER_SERVER_H_
#define SERVER_SERVER_H_

#include <boost/asio.hpp>
#include <boost/array.hpp>
#include <udp_server.h>

class Server {
 public:
  explicit Server(int port);
  ~Server();
  void run();

 private:
  int listening_port;
  boost::asio::io_service io_service;
  std::unique_ptr<UDPServer> udp_server;
};

inline Server::Server(int port) :
    listening_port(port),
    io_service(),
    udp_server(new UDPServer(this->listening_port, io_service)) {
  this->run();
  io_service.run();
}

inline void Server::run() {
  udp_server->run();
}

inline Server::~Server() {

}

#endif // SERVER_SERVER_H_
