#ifndef COMMON_DNS_H
#define COMMON_DNS_H

#include <boost/fusion/adapted/struct/adapt_struct.hpp>
#include <boost/fusion/include/adapt_struct.hpp>
#include <boost/optional.hpp>
#include <stdint.h>
#include <string>

/////////////////////////////////////////////
// -----------------------------------------
//
//    HELPER TYPES
//
// -----------------------------------------
/////////////////////////////////////////////

typedef enum : uint16_t {
  QUERY=0,
  RESPONSE=1
} QR;

typedef enum : uint16_t {
  STANDARD=0,
  INVERSE=1,
  STATUS_REQUEST=2
} Opcode;

typedef enum : uint16_t {
  NONE=0,
  FORMAT=1,
  SERVER=2,
  NO_SUCH_DOMAIN=3,
  UNSUPPORTED=4,
  REFUSED=5
} ErrorCode;

typedef enum : uint16_t {
  NUL=0,   // Empty (may be unsupported)
  A=1,     // 32-bit IPV4 address
  NS=2,    // Delegates a DNS zone to use the given authoratative name servers
  CNAME=5, // Alias of one name to another
  TXT=16   // Arbitrary readable data
} QType;

typedef enum : uint16_t {
  IN=1,    // Meant for internet
} QClass;

typedef struct {
  uint16_t offset; // Top 2 MSBs are always set
  QType qtype;
  QClass qclass;
  uint32_t ttl;
  uint16_t rd_length;
  std::string rdata;
} ResourceRecord;

/////////////////////////////////////////////
// -----------------------------------------
//
//    HEADER
//
// -----------------------------------------
/////////////////////////////////////////////

typedef struct DNS_Header_Flag {
  QR qr;
  Opcode opcode;
  bool aa;
  bool tc;
  bool rd;
  bool ra;
//uint3_t z; // Ignored
  ErrorCode rcode;

  static uint16_t serialize(const DNS_Header_Flag &flag) {
    return (flag.qr << 15) | (flag.opcode << 11) |
           (flag.aa ? 1 << 10 : 0x0000) | (flag.tc ? 1 << 9 : 0x0000) |
           (flag.rd ? 1 << 8 : 0x0000) | (flag.ra ? 1 << 7 : 0x0000) |
           (flag.rcode);
  }

  static void deserialize(DNS_Header_Flag &obj, const uint16_t flag) {
    obj.qr = (flag > 0x8000) ? RESPONSE : QUERY;
    obj.opcode = static_cast<Opcode>( (flag >> 11) & 0x0007 );
    obj.aa = (flag & 0x0400) ? true : false;
    obj.tc = (flag & 0x0200) ? true : false;
    obj.rd = (flag & 0x0100) ? true : false;
    obj.ra = (flag & 0x0080) ? true : false;
    obj.rcode = static_cast<ErrorCode>(flag & 0x000F);
  }
} DNS_Header_Flag;

typedef struct {
  uint16_t id;
  DNS_Header_Flag flags;
  uint16_t qd_count;
  uint16_t an_count;
  uint16_t ns_count;
  uint16_t ar_count;
} DNS_Header;

/////////////////////////////////////////////
// -----------------------------------------
//
//    QUESTION SECTION
//
// -----------------------------------------
/////////////////////////////////////////////

typedef struct {
  std::string qname;
  QType qtype;
  QClass qclass;
} DNS_Question;

/////////////////////////////////////////////
// -----------------------------------------
//
//    ANSWER/AUTHORITY/ADDITIONAL SECTIONS
//
// -----------------------------------------
/////////////////////////////////////////////

using DNS_Answer = std::vector<ResourceRecord>;
//using DNS_Authority = std::vector<rrecord>;
//using DNS_Additional = std::vector<rrecord>;

/////////////////////////////////////////////
// -----------------------------------------
//
//    FULL PACKET
//
// -----------------------------------------
/////////////////////////////////////////////

typedef struct {
  DNS_Header header;
  union {
    DNS_Question question;
    DNS_Answer answer;
  } data;
//  boost::optional<DNS_Authority> authority;
//  boost::optional<DNS_Additional> additional;
} DNS_Packet;


// Convert structs to Fusion sequences
BOOST_FUSION_ADAPT_STRUCT(
    DNS_Question,
    (std::string, qname),
    (QType, qtype),
    (QClass, qclass)
)

BOOST_FUSION_ADAPT_STRUCT(
    DNS_Header,
    (QR, qr),
    (Opcode, opcode),
    (bool, aa),
    (bool, tc),
    (bool, rd),
    (bool, ra),
    (bool, z),
    (ErrorCode, rcode),
    (uint16_t, qd_count),
    (uint16_t, an_count),
    (uint16_t, ns_count),
    (uint16_t, ar_count)
)

BOOST_FUSION_ADAPT_STRUCT(
    ResourceRecord,
    (uint16_t, offset),
    (QType, qtype),
    (QClass, qclass),
    (uint32_t, ttl),
    (uint16_t, rd_length),
    (char*, rdata)
)

BOOST_FUSION_ADAPT_STRUCT(
    DNS_Packet,
    (DNS_Header, header),
    (union { DNS_Question question; DNS_Answer answer; }, data)
)

#endif // COMMON_DNS_H
