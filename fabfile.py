from fabric.api import *

env.user="ec2-user"
env.hosts="ec2-52-192-144-227.ap-northeast-1.compute.amazonaws.com"
env.key_filename="./aws-rust-test.pem"
env.password=""

env.pwd=local("pwd", True)
env.repo="https://peroperopero@bitbucket.org/peroperopero/wstetris.git"

@task
def deploy():
    run("rm -rf wstetris")
    run("git clone " + env.repo)
    sudo("cp -rf ./wstetris/client/dist/* /usr/share/nginx/html")
