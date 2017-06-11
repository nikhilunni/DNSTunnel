#include <iostream>
#include <server.h>

using namespace std;

int main() {
  Server server(5000);
  server.run();

  return 0;
}
