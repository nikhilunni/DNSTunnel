#ifndef SERVER_SERVER_H_
#define SERVER_SERVER_H_

#include <boost/asio.hpp>
#include <boost/array.hpp>

using boost::asio::ip::udp;

class Server {
 public:
  static constexpr std::size_t kMaxBufferSize = 65535;

  explicit Server(int port);
  ~Server();
  void run();

 private:

  void handle_request(const boost::system::error_code& error,
		      std::size_t /*bytes_transferred*/);

  int listening_port;

  boost::asio::io_service io_service;
  udp::socket udp_socket;
  udp::endpoint remote_endpoint;
  boost::array<char, kMaxBufferSize> recv_buffer;
};

inline Server::Server(int port) :
	      listening_port(port),
	      io_service(),
	      udp_socket(this->io_service, udp::endpoint(udp::v4(), this->listening_port)) {
  this->run();
  io_service.run();
}

inline Server::~Server() {

}



#endif // SERVER_SERVER_H_
