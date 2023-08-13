# Monolith

The simplest way to run Eris is as a **monolith**. This means running your entire application in one big program.

This is most easily accomplished by running Eris on virtual machine. One of the most widely used VM systems is [Amazon Web Services EC2](https://aws.amazon.com/ec2/?nc2=h_ql_prod_cp_ec2).

You will also need a Postgres-compatible database server. If you rent a VM instance with its own storage, you can install Postgres on the same machine as your server program; otherwise, you can rent a managed database (AWS offers both [RDS](https://aws.amazon.com/rds/?nc2=h_ql_prod_db_rds) and [Aurora](https://aws.amazon.com/rds/aurora/?nc2=h_ql_prod_db_aa)).

Running as a monolith has a few key benefits:

* More predictable pricing. Virtual machines are typically rented by unit of time, so you can easily predict your monthly costs. 
* Easier routing. It's very easy to point your domain at a single machine's IP address, without any problems related to 

The tradeoff is that, with a monolith, you only get what you pay for, and you pay for everything you request.

* When your server is operating at a low utilization rate, when there are few incoming posts and few outgoing actions, you are paying for all of the hardware you have provisioned anyway.
* At times of peak load, your server may struggle to keep up with demand.

Eris' default monolith deployment is designed for vertical scaling, by using its internal resources (CPU and RAM) as efficiently as possible to handle actions like caching, load balancing, and queuing background tasks. Upgrading to a larger machine in most cloud providers is as easy as clicking a button (and paying more, of course). 