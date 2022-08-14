# Network analyzer (packet sniffer)

Authors: 
- Bellini Giuliano (s294739)
- Canepari Cristiano Marco (s304808)

## Contents

- [Introduction](#introduction)

- [Command line options](#command-line-options)

- [Textual report structure](#textual-report-structure)
  + [Report header](#report-header)
  + [Report address:port list](#report-addresses-list)
  
- [Error conditions](#error-conditions)
  + [Wrong command line option specification](#wrong-command-line-options-specification)
  + [Permissions errors](#permissions-errors)
  
 
## Introduction

Aim of the project is to intercept incoming and outgoing traffic through a user specified network interface of his computer.

The application will generate a textual report, providing statistics about the observed network packets.

Below you can find the available command line options, the structure of the report file generated by the application, the possible error conditions that may occurr, and other useful informations.


## Command line options

The executable file path is ```packet_sniffer/target/debug/packet_sniffer```.

 - ```-a, --adapter```
 
          Name of the network adapter to be inspected, if omitted the default adapter is chosen.
          
          If a non-existing adapter is provided, the application raises an error and terminates.
          
          This option must be followed by a textual value.
 
 - ```-d, --device-list```
 
           Prints list of the available network interfaces.
           
           Immediately terminates the program.
           
           This option does not need to be followed by a value.
 
 - ```-h, --highest-port```
 
          Sets the maximum port value to be considered, if omitted there is not ports higher bound.

          If the highest-port provided value is lower than the lowest-port provided value, the application raises an error and terminates.
          
          This option must be followed by an integer value between 0 and 65535. 
          
           ```default: 65535```
 
 -  ```-i, --interval```
 
           Sets the interval of time between report updates (value in seconds).
           
           This option must be followed by a positive integer value.
 
           ```default: 5```
 
 - ```-l, --lowest-port```
 
          Sets the minimum port value to be considered, if omitted there is not ports lower bound.

          If the lowest-port provided value is higher than the highest-port provided value, the application raises an error and terminates.

          This option must be followed by an integer value between 0 and 65535. 

          ```default: 0```
 
 - ```-m, --minimum-packets```
 
          Sets the minimum value of transited packets for an address:port to be printed in the report.

          This option must be followed by a positive integer value.

          ```default: 0```

- ```-n, --network-layer-filter```

          Filters packets on the basis of the IP version address (IPv4 or IPv6).

          If a string different from "IPv4" or "IPv6" is provided (not case sensitive), the application raises an error and terminates.

          This option must be followed by a textual value.
            
          ```default: "no filter"```
 
 - ```-o, --output-file```
 
          Name of output file to contain the textual report, if omitted a default file is chosen.

          This option must be followed by a textual value.

          ```default: report.txt```

- ```-t, --transport-layer-filter```

          Filters packets on the basis of the transport layer protocol (TCP or UDP).

          If a string different from "TCP" or "UDP" is provided (not case sensitive), the application raises an error and terminates.

          This option must be followed by a textual value.

          ```default: "no filter"```



## Textual report structure

In this section is reported the structure of the output report file generated, to help the users better understand and interpret it.

### Report header

The first section of the textual report contains an header summarizing different useful informations.

![Screenshot](./img/report_part_1.png)

First of all, it specifies the name of the network adapter analyzed during the sniffing process.

Then there is a detail about the initial timestamp of the sniffing process, the last timestamp in which the report was updated, the frequency of updates and the number of times the report was updated (re-written from scratch with updated data).

Finally, it describes the status of the possible filters applicable by the user through the command line: minimum number of packets for and address:port pair to be printed in the report, IP address version, transport layer protocol and port minimum and maximum number.

Note that an application layer protocol filter is not provided since the user can use the lowest and highest port options to this purpose (e.g., to filter DNS traffic a user can specify ```packet_sniffer -l 53 -h 53```, to filter HTTPS traffic a user can specify ```packet_sniffer -l 443 -h 443``` and so on).

### Report addresses list

The second section of the textual report is dedicated to the packets stream analysis for each address:port pair.

This analysis results in a list in which each element represents an address:port pair with the relative statistics.

![Screenshot](./img/report_part_2.png)

For each element it is reported the amount of sent data (relatively to packets in which the address:pair is the source) and received data (relatively to packets in which the address:port is the destination) measured in number of packets and in number of bytes.

For each address:port pair are reported the first and the last timestamp in which a packet was transmitted from/to that address:port.

Level 4 and level 7 carried protocols are also described (respectively transport layer and application layer protocols).

Both the transport layer protocols and application layer protocols fields could report a single or multiple protocols for each address:port pair, based on the traffic type.

Specifically, the transport layer protocols field is based on an Enum with only two values (TCP and UDP), while the application layer protocols field is based on an Enum with some of the most common level 7 protocols (listed in the table below); please note that application level protocols are just inferred from the transport port numbers.

|Port number(s)|Application protocol  |  Description |
|--|--|--|
| 20, 21 | FTP |File Transfer Protocol |
|22|SSH |Secure Shell |
|23|Telnet |Telnet |
|25|SMTP |Simple Mail Transfer Protocol |
|53|DNS |Domain Name System |
|67, 68|DHCP |Dynamic Host Configuration Protocol |
|69|TFTP |Trivial File Transfer Protocol |
|80|HTTP |Hypertext Transfer Protocol |
|110|POP |Post Office Protocol |
|123|NTP |Network Time Protocol |
|137, 138, 139|NetBIOS |NetBIOS |
|143|IMAP |Internet Message Access Protocol |
|161,162|SNMP |Simple Network Management Protocol |
|179|BGP |Border Gateway Protocol |
|389|LDAP |Lightweight Directory Access Protocol |
|443|HTTPS |Hypertext Transfer Protocol over SSL/TLS |
|636|LDAPS |Lightweight Directory Access Protocol over TLS/SSL |
|989, 990|FTPS |File Transfer Protocol over TLS/SSL |


## Error conditions

In this section are reported the errors that may occur while the application is running.

### Wrong command line options specification


- **Not existing adapter name**

&emsp;&emsp;&emsp; If a non-existing adapter name is provided, the application raises an error and terminates.

&emsp;&emsp;&emsp; In this case the application will suggest to use the ```-d``` option to print on the standard output a list of the available devices.

&emsp;&emsp;&emsp; ```packet_sniffer -d``` prints a list of all the available network adapters names and addresses, as in the example that follows.

&emsp;&emsp;&emsp; ![Screenshot](./img/device_list.png)


- **Invalid highest port number**

&emsp;&emsp;&emsp; If the provided highest port number is not an integer in the range ```0..=65535``` the program raises an error and terminates.

&emsp;&emsp;&emsp; If also the lowest port number is specified and ```highest_port < lowest_port == true``` the program raises an error and terminates.


- **Invalid interval value**

&emsp;&emsp;&emsp; If the provided interval value is not an integer in the range ```1..=u64::MAX``` the program raises an error and terminates.


- **Invalid lowest port number**

&emsp;&emsp;&emsp; If the provided lowest port number is not an integer in the range ```0..=65535``` the program raises an error and terminates.

&emsp;&emsp;&emsp; If also the highest port number is specified and ```highest_port < lowest_port == true``` the program raises an error and terminates.


- **Invalid minimum packets value**

&emsp;&emsp;&emsp; If the provided minimum packets value is not an integer in the range ```0..=u32::MAX``` the program raises an error and terminates.


- **Invalid network layer protocol filter**

&emsp;&emsp;&emsp; If a string different from "IPv4", "IPv6" or "no filter" is provided (not case sensitive), the application raises an error and terminates.

&emsp;&emsp;&emsp; Note that not including the ```-n``` option is equal to provide ```-n "no filter"```.


- **Invalid ouput file extension**

&emsp;&emsp;&emsp; There is no particular limitation on the output file name.

&emsp;&emsp;&emsp; However, if an invalid file extension is provided the file may result unreadable if the extension is not subsequently removed.


- **Invalid transport layer protocol filter**

&emsp;&emsp;&emsp; If a string different from "TCP", "UDP" or "no filter" is provided (not case sensitive), the application raises an error and terminates.

&emsp;&emsp;&emsp; Note that not including the ```-t``` option is equal to provide ```-t "no filter"```.


### Permissions errors

- **PcapError: Permission denied**

&emsp;&emsp;&emsp; You may incur in this error if you have not the privilege to open a network adapter. Full error is reported below.

&emsp;&emsp;&emsp; ![Screenshot](./img/error_permissions.png)

&emsp;&emsp;&emsp; To solve this error you can execute the following commands:

&emsp;&emsp;&emsp; ```cd /dev```

&emsp;&emsp;&emsp; ```sudo chown <username>:admin bp*```

&emsp;&emsp;&emsp; Where \<username\> can be retrieved with the command ```whoami```.

&emsp;&emsp;&emsp; You will be requested to insert your system password.
