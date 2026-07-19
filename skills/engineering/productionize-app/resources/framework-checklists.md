# Framework & Deployment Checklists

Reference material for the productionize-app skill. Apply the block that matches the app's framework (during the Framework-Specific Optimizations phase) and the block that matches the deployment target (during the Deployment Preparation phase).

## Framework-Specific Optimizations

### Flutter

- Implement SharedPreferences for persistent caching
- Add proper error handling for network requests
- Optimize widget rebuilds and state management
- Add platform-specific configurations (iOS/Android)
- Implement proper navigation and state restoration

### React/Next.js

- Implement proper state management (Redux, Zustand, Context)
- Add error boundaries and suspense loading
- Optimize bundle size and code splitting
- Implement proper caching (SWR, React Query)
- Add SEO and meta tag optimizations

### Node.js

- Add proper middleware for error handling and logging
- Implement rate limiting and security headers
- Optimize database queries and connections
- Add health checks and monitoring endpoints
- Implement proper environment configuration

### Python

- Add proper error handling and logging
- Implement caching (Redis, in-memory)
- Optimize database queries and ORM usage
- Add input validation and sanitization
- Implement proper testing and CI/CD

## Deployment Preparation

### TestFlight (iOS)

- Verify App Store Connect configurations
- Test provisioning profiles and certificates
- Validate Info.plist settings
- Create build and upload scripts
- Prepare app description and screenshots

### Google Play (Android)

- Configure Play Console settings
- Test signing configurations
- Validate manifest permissions
- Create release notes and store listing
- Test different device configurations

### Web Deployment (Vercel/Netlify/AWS)

- Configure build scripts and environment variables
- Set up domain and SSL certificates
- Test deployment pipeline
- Configure CDN and caching headers
- Set up monitoring and analytics

### Container Deployment (Docker/Kubernetes)

- Create optimized Dockerfiles
- Configure health checks and resource limits
- Set up environment variable management
- Test scaling and load balancing
- Configure logging and monitoring
