
import React, { useRef, useEffect } from 'react';
import * as THREE from 'three';

interface VolumetricSceneProps {
  intensity: number;
  emotion: 'NEUTRAL' | 'ENGAGED' | 'PROCESSING' | 'CRITICAL';
}

const VolumetricScene: React.FC<VolumetricSceneProps> = ({ intensity, emotion }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const mouse = useRef({ x: 0, y: 0 });
  const targetMouse = useRef({ x: 0, y: 0 });

  useEffect(() => {
    if (!containerRef.current) return;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    const renderer = new THREE.WebGLRenderer({ 
      alpha: true, 
      antialias: true, 
      powerPreference: "high-performance" 
    });
    
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.setClearColor(0x000000, 0); 
    containerRef.current.appendChild(renderer.domElement);

    // Lattices
    const gridColor = 0x0ea5e9;
    const gridOpacity = 0.03;
    
    const floor = new THREE.GridHelper(400, 80, gridColor, 0x000000);
    floor.position.y = -25;
    floor.material.transparent = true;
    floor.material.opacity = gridOpacity;
    scene.add(floor);

    const ceiling = new THREE.GridHelper(400, 80, gridColor, 0x000000);
    ceiling.position.y = 25;
    ceiling.material.transparent = true;
    ceiling.material.opacity = gridOpacity;
    scene.add(ceiling);

    // Neural Cloud
    const count = 3000;
    const geom = new THREE.BufferGeometry();
    const pos = new Float32Array(count * 3);
    for (let i = 0; i < count * 3; i++) {
      pos[i] = (Math.random() - 0.5) * 200;
    }
    geom.setAttribute('position', new THREE.BufferAttribute(pos, 3));
    const mat = new THREE.PointsMaterial({
      color: 0x22d3ee,
      size: 0.15,
      transparent: true,
      opacity: 0.1,
      blending: THREE.AdditiveBlending
    });
    const neuralPoints = new THREE.Points(geom, mat);
    scene.add(neuralPoints);

    camera.position.z = 40;

    const handleMouseMove = (event: MouseEvent) => {
      targetMouse.current.x = (event.clientX / window.innerWidth) * 2 - 1;
      targetMouse.current.y = -(event.clientY / window.innerHeight) * 2 + 1;
    };
    window.addEventListener('mousemove', handleMouseMove);

    const animate = () => {
      requestAnimationFrame(animate);
      mouse.current.x += (targetMouse.current.x - mouse.current.x) * 0.03;
      mouse.current.y += (targetMouse.current.y - mouse.current.y) * 0.03;

      const time = Date.now() * 0.001;
      neuralPoints.rotation.y += 0.0002;
      neuralPoints.rotation.x += 0.0001;

      camera.position.x = mouse.current.x * 15;
      camera.position.y = mouse.current.y * 15;
      camera.lookAt(0, 0, 0);

      renderer.render(scene, camera);
    };
    animate();

    const handleResize = () => {
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(window.innerWidth, window.innerHeight);
    };
    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('resize', handleResize);
      containerRef.current?.removeChild(renderer.domElement);
    };
  }, [intensity, emotion]);

  return <div ref={containerRef} className="fixed inset-0 z-[-50] pointer-events-none overflow-hidden" />;
};

export default VolumetricScene;
