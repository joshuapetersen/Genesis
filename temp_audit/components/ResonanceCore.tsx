
import React, { useEffect, useRef } from 'react';
import * as THREE from 'three';

interface ResonanceCoreProps {
  color?: string;
  vibration?: number;
}

const ResonanceCore: React.FC<ResonanceCoreProps> = ({ color = '#00A3FF', vibration = 0 }) => {
  const mountRef = useRef<HTMLDivElement>(null);
  const pointsRef = useRef<THREE.Points>(null);
  const initialPositionsRef = useRef<Float32Array | null>(null);
  const lerpedVibration = useRef(0);

  useEffect(() => {
    if (!mountRef.current) return;

    const width = window.innerWidth;
    const height = window.innerHeight;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(width, height);
    mountRef.current.appendChild(renderer.domElement);

    const coreColor = new THREE.Color(color);

    // Particle Cloud (The Sovereign Core)
    const geometry = new THREE.BufferGeometry();
    const vertices = [];
    const count = 6000;
    const radius = 2.0;
    
    for (let i = 0; i < count; i++) {
      // Golden spiral distribution for a more organic shell
      const theta = Math.random() * Math.PI * 2;
      const phi = Math.acos(2 * Math.random() - 1);
      const x = radius * Math.sin(phi) * Math.cos(theta);
      const y = radius * Math.sin(phi) * Math.sin(theta);
      const z = radius * Math.cos(phi);
      vertices.push(x, y, z);
    }
    
    const vertexArray = new Float32Array(vertices);
    geometry.setAttribute('position', new THREE.BufferAttribute(vertexArray, 3));
    initialPositionsRef.current = vertexArray.slice();

    const material = new THREE.PointsMaterial({
      color: coreColor,
      size: 0.035,
      transparent: true,
      opacity: 0.8,
      blending: THREE.AdditiveBlending,
      depthWrite: false,
    });

    const points = new THREE.Points(geometry, material);
    pointsRef.current = points;
    scene.add(points);

    camera.position.z = 6;

    const animate = () => {
      requestAnimationFrame(animate);
      
      // Smooth out the vibration input
      lerpedVibration.current += (vibration - lerpedVibration.current) * 0.12;
      const v = lerpedVibration.current;
      const time = Date.now() * 0.001;
      
      // Global Core Rotations
      points.rotation.y += 0.002 + (v * 0.03);
      points.rotation.x += 0.001 + (v * 0.01);

      // Particle-level Waveform Vibration
      if (initialPositionsRef.current && pointsRef.current) {
        const positions = pointsRef.current.geometry.attributes.position.array as Float32Array;
        const initials = initialPositionsRef.current;
        
        for (let i = 0; i < positions.length; i += 3) {
          const ix = initials[i];
          const iy = initials[i+1];
          const iz = initials[i+2];
          
          // Calculate distance for radial wave modulation
          const dist = Math.sqrt(ix*ix + iy*iy + iz*iz);
          
          // Waveform math: combines baseline breathing with high-freq vibration noise
          const breath = Math.sin(time * 2 + dist) * 0.05;
          const speechVibe = Math.sin(time * 30 + i) * v * 0.4;
          const expansion = 1 + breath + (v * 0.7);
          
          positions[i] = ix * (expansion + speechVibe);
          positions[i+1] = iy * (expansion + speechVibe);
          positions[i+2] = iz * (expansion + speechVibe);
        }
        pointsRef.current.geometry.attributes.position.needsUpdate = true;
      }
      
      // Dynamic Opacity based on volume
      material.opacity = 0.4 + (v * 0.6);
      
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
      window.removeEventListener('resize', handleResize);
      if (mountRef.current && renderer.domElement.parentNode) {
        mountRef.current.removeChild(renderer.domElement);
      }
      geometry.dispose();
      material.dispose();
    };
  }, []);

  // Update color dynamically to match emotional resonance
  useEffect(() => {
    if (pointsRef.current) {
      const targetColor = new THREE.Color(color);
      (pointsRef.current.material as THREE.PointsMaterial).color.lerp(targetColor, 0.15);
    }
  }, [color]);

  return (
    <div ref={mountRef} className="absolute inset-0 flex items-center justify-center pointer-events-none z-0 overflow-hidden" />
  );
};

export default ResonanceCore;
