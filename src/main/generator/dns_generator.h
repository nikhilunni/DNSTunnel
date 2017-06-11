#ifndef GENERATOR_UDP_GENERATOR_H
#define GENERATOR_UDP_GENERATOR_H

class DNSGenerator {
 public:
  explicit DNSGenerator();
  ~DNSGenerator();
  bool generate(vector<unsigned char>, DNS_Packet);
}

#endif // GENERATOR_UDP_GENERATOR_H
