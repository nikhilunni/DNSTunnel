#include <boost/serialization/vector.hpp>
#include <../common/dns.h>

BOOST_SERIALIZATION_SPLIT_FREE(DNS_Header_Flag)

namespace boost {
namespace serialization {

template<class Archive>
void serialize(Archive &ar, ResourceRecord &rrecord, const unsigned int version) {
  if (Archive::is_saving::value) {
    ar & (rrecord.offset | 0xC000);
  } else if (Archive::is_loading::value) {
    uint16_t val;
    ar & val;
    rrecord.offset = val & 0x3FFF;
  }
  ar & rrecord.qtype;
  ar & rrecord.qclass;
  ar & rrecord.ttl;
  ar & rrecord.rd_length;
  ar & rrecord.rdata;
}

template<class Archive>
void save(Archive &ar, DNS_Header_Flag &flags, const unsigned int version) {
  ar & DNS_Header_Flag::serialize(flags);
}

template<class Archive>
void load(Archive &ar, DNS_Header_Flag &flags, const unsigned int version) {
  uint16_t byte_values;
  ar & byte_values;
  DNS_Header_Flag::deserialize(flags, byte_values);
}

template<class Archive>
void serialize(Archive &ar, DNS_Header &header, const unsigned int version) {
  ar & header.id;
  ar & header.flags;
  ar & header.qd_count;
  ar & header.an_count;
  ar & header.ns_count;
  ar & header.ar_count;
}

template<class Archive>
void serialize(Archive &ar, DNS_Question &question, const unsigned int version) {
  ar & question.qname;
  ar & question.qtype;
  ar & question.qclass;
}

template<class Archive>
void serialize(Archive &ar, DNS_Packet &packet, const unsigned int version) {
  ar & packet.header;
  ar & packet.data;
}

} // namespace boost
} // namespace serialization
