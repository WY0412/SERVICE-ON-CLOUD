aws ecr get-login-password --region us-east-1 | docker login --username AWS  --password-stdin 851725245278.dkr.ecr.us-east-1.amazonaws.com
# aws ecr create-repository     --repository-name qrcode-ecr --region us-east-1
docker build -f web/qrcode/Dockerfile -t 851725245278.dkr.ecr.us-east-1.amazonaws.com/qrcode-ecr web/qrcode/
# docker run -p 8080:8080  851725245278.dkr.ecr.us-east-1.amazonaws.com/qrcode-ecr
docker push 851725245278.dkr.ecr.us-east-1.amazonaws.com/qrcode-ecr
# kubectl apply -f Ingress/ingress.yaml 
helm install qrcode k8s/helm/qrcode
kubectl get ingress
kubectl get svc
kubectl get pods
# by using kubectl top pod / kubectl top node kubectl describe pods kubectl describe nodes
# helm uninstall qrcode
# kubectl scale --replicas=12 -f k8s/helm/qrcode/templates/deployment.yaml