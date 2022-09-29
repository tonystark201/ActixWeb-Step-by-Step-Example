#!bin/bash
A=`ps -ef | grep nginx | grep -v grep | wc -l`
if [ $A -eq 0 ];then
    /user/sbin/nginx
    sleep 2
    if [ `ps -ef | grep nginx | grep -v grep | wc -l` -eq 0 ];then
        ps -ef|grep keepalived|grep -v grep|awk '{print $2}'|xargs kill -9
    fi
fi