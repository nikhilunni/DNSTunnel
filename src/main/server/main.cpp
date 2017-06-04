#include <iostream>
#include <server.h>

using namespace std;

int main() {
  dns_tunnel::server::Server server(5000);
  server.run();

  return 0;
}
