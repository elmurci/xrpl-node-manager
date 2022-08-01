# Build frontend application
FROM builder AS ui
WORKDIR /panda-planner/frontend-planner/
COPY /frontend-planner/package*.json .
RUN npm install --legacy-peer-deps
COPY /frontend-planner/ .
RUN npm run build

FROM nginx:stable-alpine AS nginx
COPY --from=frontend /xrpl-node-manager-ui/frontend-planner/dist/ /usr/share/nginx/html/
#COPY ./default.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;" ]