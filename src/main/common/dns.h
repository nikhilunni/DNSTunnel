#ifndef COMMON_DNS_H
#define COMMON_DNS_H

#include <boost/optional.hpp>
#include <stdint.h>
#include <string>

namespace common {

/////////////////////////////////////////////
// -----------------------------------------
//
//    HELPER TYPES
//
// -----------------------------------------
/////////////////////////////////////////////

typedef enum {
  QUERY=0,
  RESPONSE=1
} QR;

typedef enum {
  STANDARD=0,
  INVERSE=1,
  STATUS_REQUEST=2
} Opcode;

typedef enum {
  NONE=0,
  FORMAT=1,
  SERVER=2,
  NO_SUCH_DOMAIN=3,
  UNSUPPORTED=4,
  REFUSED=5
} ErrorCode;

typedef enum {
  NUL=0,   // Empty (may be unsupported)
  A=1,     // 32-bit IPV4 address
  NS=2,    // Delegates a DNS zone to use the given authoratative name servers
  CNAME=5, // Alias of one name to another
  TXT=16   // Arbitrary readable data
} QType;

typedef enum {
  IN=1,    // Meant for internet
} QClass;

typedef struct QNameOffset {
 public:
  QNameOffset(uint16_t offset) : offset(offset) {};
  int get_offset() { return offset | 0xC000; } // Top 2 MSB are 1
 private:
  uint16_t offset;
} QNameOffset;

typedef struct {
  QNameOffset offset;
  QType qtype;
  QClass qclass;
  uint32_t ttl;
  uint16_t rd_length;
  char *rdata;
} ResourceRecord;

/////////////////////////////////////////////
// -----------------------------------------
//
//    HEADER
//
// -----------------------------------------
/////////////////////////////////////////////

typedef struct {
  uint16_t id;
  QR qr;
  Opcode opcode;
  bool aa;
  bool tc;
  bool rd;
  bool ra;
  bool z; // Ignored
  ErrorCode rcode;
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
  };
//  boost::optional<DNS_Authority> authority;
//  boost::optional<DNS_Additional> additional;
} DNS_Packet;


} //namespace common

#endif // COMMON_DNS_H
