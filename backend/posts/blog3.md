# First steps towards a home lab ☁️

&nbsp;

_Wed 12 Oct, 2022_

&nbsp;

Recently I have bought an Intel NUC, primarily to self-host Home Assistant and some other software that you would expect in a [home lab](https://www.reddit.com/r/homelab/). My first step was to install Ubuntu Server with a [bootable usb](https://ubuntu.com/tutorials/create-a-usb-stick-on-macos#1-overview). For this step it is useful to connect a keyboard and a monitor. Once that was setup I generated a SSH key and shared the public part of the key with my other computer. This allows me to connect to the Intel NUC remotely over my home network, so it can be stored away.

&nbsp;

The next step was to open up a port on my home network, so I was able to reach to NUC over the public IP. Obviously, this introduces some security risks so I followed the remedies prescribed by the [Home Assistant blog](https://www.home-assistant.io/blog/2015/12/13/setup-encryption-using-lets-encrypt/). Doing so encrypts traffic and allows me to access Home Assistant through a domain name rather than an IP. This also makes it possible to integrate third party services such as [Google Assistant](https://www.home-assistant.io/integrations/google_assistant/).

&nbsp;

Thus far I have some additional services running, such as Prometheus and Grafana, to monitor my hardware in a [fancy dashboard](https://grafana.com/grafana/dashboards/). In the future I'm planning to make my setup a bit more resilient with [Ansible](https://www.ansible.com/), which is a tool to automate management of remote machines, such as the NUC. Ansible needs to run on a control node, my personal computer, and controls the remote node, the NUC, to install Ubuntu dependencies and some configuration. This would make the software side of things fully disposable, which is quite comfortable :smiley:.
