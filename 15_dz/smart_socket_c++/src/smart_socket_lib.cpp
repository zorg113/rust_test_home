#include <iostream>
#include <string>
#include <sstream>
#include <memory>
#include <cstring>

enum Status{
    On,
    Off
};

class SmartSocket {
    std::string status_;
    double power_ = 0.0;
public:
    void set_status(Status status){
        switch (status)
        {
        case Status::On:
            status_ = "On" ;
            power_ = 220.;
            break;
        case Status::Off:
            status_ = "Off";
            power_ = 0.0;
            break;    
        };               
    }

    std::string get_status(void) {
       std::ostringstream s_out;
       s_out<<"Status: "<<status_<<" Power: "<< power_ <<"B";
       return s_out.str();
    }
};

SmartSocket sm_sock;


extern "C" {
    void ex_set_status(int status){
        sm_sock.set_status(static_cast<Status>(status));
    }

    void get_status(char* buf, size_t b_size){
        if (buf != NULL) {
            std::strncpy(buf, sm_sock.get_status().c_str(), b_size);
        }
    }
}

