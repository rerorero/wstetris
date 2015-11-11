from fabric.api import *
from fabric.contrib.files import *

env.user="ec2-user"
env.hosts="ec2-52-192-144-227.ap-northeast-1.compute.amazonaws.com"
env.key_filename="./aws-rust-test.pem"
env.password=""

env.pwd=local("pwd", True)
env.repo="https://peroperopero@bitbucket.org/peroperopero/wstetris.git"
env.pid="~/tetris.pid"

@task
def deploy():
    run("rm -rf wstetris")
    run("git clone " + env.repo)
    sudo("cp -rf ./wstetris/client/dist/* /usr/share/nginx/html")

@task
def start():
    stop()
    with cd("./wstetris/server"):
        run("nohup cargo run > /dev/null 2>&1 & echo $! > " + env.pid, pty=False)

@task
def test():
    with cd("./wstetris/server"):
        run("ls -al")
        run("cargo run")
    
@task
def stop():
    if exists(env.pid):
        run('kill -s 2 $(cat {0}) && rm {0}'.format(env.pid))
    
