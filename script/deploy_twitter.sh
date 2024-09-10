aws ecr get-login-password --region us-east-1 | docker login --username AWS  --password-stdin 851725245278.dkr.ecr.us-east-1.amazonaws.com
# aws ecr create-repository     --repository-name twitter-ecr --region us-east-1
docker build -f web/twitter/Dockerfile -t 851725245278.dkr.ecr.us-east-1.amazonaws.com/twitter-ecr web/twitter/
# docker run -p 8080:8080  851725245278.dkr.ecr.us-east-1.amazonaws.com/twitter-ecr
docker push 851725245278.dkr.ecr.us-east-1.amazonaws.com/twitter-ecr
# kubectl apply -f Ingress/ingress.yaml 
helm install twitter k8s/helm/twitter
kubectl get ingress
kubectl get svc
kubectl get pvc
kubectl get pods

# by using kubectl top pod / kubectl top node kubectl describe pods kubectl describe nodes
# helm uninstall twitter
# kubectl scale --replicas=12 -f k8s/helm/twitter/templates/deployment.yaml
# kubectl describe storageclass
# kubectl get pv
# kubectl get pvc mysql-pvc
# kubectl exec -it mysql-6bf994f6c4-lm6fn --
# kubectl cp ../data/combinedScores_and_descriptions.sql mysql-6bf994f6c4-lm6fn:/tmp/dump.sql
# kubectl get volumesnapshot
# kubectl port-forward mysql-579dc8595-6ljhd 3307:3306

# kubectl exec -it [MYSQL_POD_NAME] -- mysql -uroot -p

# Run these to completely remove the volumesnapshot and volumesnapshotcontent
# kubectl delete volumesnapshot test-snapshot-retain
# kubectl delete volumesnapshotcontent test-snapshot-content
# kubectl patch volumesnapshot test-snapshot-retain -p '{"metadata":{"finalizers":null}}' --type=merge
# kubectl patch volumesnapshotcontent test-snapshot-content  --type json --patch='[ { "op": "remove", "path": "/metadata/finalizers" } ]'
# kubectl delete pv mysql-pv
# kubectl delete pvc mysql-pvc
