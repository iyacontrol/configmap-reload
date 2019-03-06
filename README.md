# configmap-reload
this repo is used for configmap in k8s reload.

## 使用方法

```yaml
---
apiVersion: v1
kind: ConfigMap
metadata:
  labels:
    app: configmap-reload
  name: configmap-reload-cm
data:
  test.ini: |-
    key: a

---
kind: Deployment
apiVersion: apps/v1
metadata:
  name: configmap-reload
  labels:
    app: configmap-reload
spec:
  replicas: 1
  selector:
    matchLabels:
      app: configmap-reload
  template:
    metadata:
      labels:
        app: configmap-reload
    spec:
      volumes:
      - name: config
        configMap:
          name: configmap-reload-cm
      containers:
      - name: configmap-reload
        image: 'iyacontrol/configmap-reload:v0.1'
        command:
          - /usr/local/bin/configmap-reload
        args:
          - -l
          - debug
          - -p 
          - /etc/test/  
          - -c 
          - '200' 
          - -u 
          - https://www.baidu.com
        volumeMounts:
        - name: config
          mountPath: /etc/test/
        imagePullPolicy: Always

---

```

## 资源消耗

```sh
[root@ip-172-xx-xx-10 src]# kubectl top pods
NAME                                CPU(cores)   MEMORY(bytes)
configmap-reload-6bbbb8b45b-7zg2x   0m           1Mi
```
