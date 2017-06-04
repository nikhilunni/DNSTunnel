#ifndef SERVER_UDP_SERVER_H
#define SERVER_UDP_SERVER_H

#include <boost/asio.hpp>
#include <boost/array.hpp>
#include "../common/constants.h"

namespace dns_tunnel {
namespace udp {

class UDPServer {
 public:
  explicit UDPServer(int port, boost::asio::io_service& io_service);
  ~UDPServer();
  void run();
 private:
  int listening_port;
  boost::asio::ip::udp::socket socket;
  boost::asio::ip::udp::endpoint endpoint;
  boost::array<char, ::dns_tunnel::udp::kMaxBufferSize> recv_buffer;

  void handle_request(const boost::system::error_code& error,
                      std::size_t /*bytes_transferred*/);
};

inline UDPServer::UDPServer(int port, boost::asio::io_service& io_service) :
    listening_port(port),
    socket(io_service, boost::asio::ip::udp::endpoint(boost::asio::ip::udp::v4(),
                                                      this->listening_port)) {
  this->run();
}

inline UDPServer::~UDPServer() {

}

} // namespace udp
} // namespace dns_tunnel


#endif // SERVER_UDP_SERVER_H
