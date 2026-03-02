detector_service:
  podman_container.running:
    - name: detector
    - image: {{ salt['pillar.get']('detector:image') }}
    - ports:
      - 9090:9090
    - volumes:
      - /var/log/detector:/var/log/detector
    - restart_policy: unless-stopped
    - require:
      - file /etc/detector/config.yaml
