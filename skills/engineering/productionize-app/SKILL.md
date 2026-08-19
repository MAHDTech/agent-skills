---
name: productionize-app
description: Transform an application into a production-ready deployment through systematic analysis, hardening, and framework-specific optimization. Use when preparing an app to ship - auditing readiness, tightening config, and getting it deployable to a target platform.
disable-model-invocation: true
context: fork
agent: general-purpose
---

# Productionize App

When asked to productionize an application for deployment:

## Overview

Transform development code into production-ready applications through systematic analysis, improvement, and deployment preparation. Supports multiple frameworks and deployment targets.

## Usage Patterns

```bash
/productionize-app                          # Auto-detect framework, general production prep
/productionize-app flutter testflight       # Flutter app for TestFlight
/productionize-app react vercel             # React app for Vercel deployment
/productionize-app nodejs docker            # Node.js app for Docker deployment
/productionize-app python heroku            # Python app for Heroku
/productionize-app vue netlify              # Vue app for Netlify
```

## Workflow

### 1. **Initial Analysis & Planning**

- **Always start by drafting a comprehensive task list** to track the work (use your task-tracking tool if you have one)
- **Breadth AND Depth Navigation**: Look beyond immediate scope to understand:
  - Related systems that might be affected
  - Broader architectural implications
  - Integration points and dependencies
  - Edge cases and potential failure modes
  - Long-term maintenance considerations
- Analyze codebase structure and identify the framework/stack
- Read existing documentation (README, package.json, pubspec.yaml, etc.)
- Identify deployment target and requirements
- Create systematic task breakdown covering all productionization phases
- **Proactive Question Analysis**: Identify critical production questions the user hasn't asked yet

### 2. **Codebase Analysis Phase**

- **Architecture review**: Analyze project structure, dependencies, and patterns with methodical thoroughness
- **Comprehensive Production Readiness Audit**: Systematically identify potential issues:
  - Hardcoded values and missing environment configurations
  - Missing error handling and edge case coverage
  - Performance bottlenecks and optimization opportunities
  - Security vulnerabilities and data exposure risks
  - Missing caching and state management
  - API pagination and data fetching issues
  - UI/UX polish and accessibility concerns
  - Rate limiting and abuse protection
  - Offline support and network failure handling
  - Data validation and sanitization
  - Memory leaks and resource cleanup
  - Cross-platform compatibility issues
  - Third-party service dependencies and fallbacks
- **Framework-specific checks**: Apply framework-appropriate best practices with deep domain knowledge
- **Critical Questions Analysis**: Document what production concerns the user should consider but hasn't addressed

### 3. **Implementation Phase**

Execute improvements systematically, updating your task-tracking tool as you go:

#### **Configuration & Environment**

- Add environment-based configurations (dev/staging/prod)
- Implement feature toggles and debug modes
- Secure API keys and sensitive data
- Add configuration validation

#### **Performance & Reliability**

- Implement proper caching mechanisms (memory, disk, network)
- Add error handling and retry logic
- Optimize network requests and pagination
- Add loading states and offline handling
- Implement proper state persistence

#### **User Experience**

- Polish UI/UX with loading indicators and feedback
- Add proper error messages and user guidance
- Implement accessibility improvements
- Test edge cases and error scenarios

#### **Code Quality**

- Add comprehensive logging and monitoring
- Implement proper testing coverage
- Clean up technical debt and code smells
- Add documentation and comments

### 4. **Framework-Specific Optimizations**

Apply the checklist for the detected framework (Flutter, React/Next.js, Node.js, Python) from [resources/manual/framework-checklists.md](resources/manual/framework-checklists.md#framework-specific-optimizations).

### 5. **Documentation Phase**

Create comprehensive production-ready documentation:

#### **README.md Structure** (based on proven patterns)

```markdown
# Project Name - Production Ready

_Brief compelling description with value proposition_

## Quick Start

- One-command setup instructions
- Environment requirements
- Configuration steps

## Features

- Core functionality overview
- Production-ready capabilities
- Toggle configurations

## Deployment

- Platform-specific deployment guides
- Environment variable setup
- Testing and validation steps

## Development

- Local development setup
- Testing procedures
- Contributing guidelines

## Architecture

- High-level system overview
- Key design decisions
- Production considerations
```

#### **Additional Documentation**

- Deployment checklists for each target platform
- API documentation if applicable
- Troubleshooting guides
- Performance optimization notes

### 6. **Deployment Preparation Phase**

Apply the checklist for the chosen deployment target (TestFlight, Google Play, Web, or Container) from [resources/manual/framework-checklists.md](resources/manual/framework-checklists.md#deployment-preparation).

### 7. **Quality Assurance Checklist**

Before marking productionization complete:

- [ ] All configurations are environment-aware
- [ ] Error handling covers edge cases
- [ ] Caching is implemented and tested
- [ ] Performance is optimized for target platforms
- [ ] Security best practices are followed
- [ ] Documentation is comprehensive and accurate
- [ ] Deployment process is tested and reliable
- [ ] Monitoring and logging are configured
- [ ] User experience is polished and accessible
- [ ] All tracked checklist tasks are completed

### 8. **Proactive Question Analysis & Next Steps**

After completing productionization, **ALWAYS** provide:

#### **Critical Questions the User Should Have Asked**

Systematically identify and present production concerns the user hasn't considered:

**Example Framework-Specific Questions:**

- **Flutter**: "Have you considered rate limiting for API calls? What happens when the device goes offline? How will you handle app store review rejections?"
- **React/Web**: "What's your strategy for SEO and social media previews? How will you handle bot traffic? What about GDPR compliance for user data?"
- **Node.js**: "How will you monitor server health in production? What's your database backup strategy? How will you handle traffic spikes?"
- **Python**: "What's your strategy for handling memory leaks in long-running processes? How will you manage database migrations in production?"

#### **Actionable Next Steps Checklist**

Provide a prioritized, concrete list of post-productionization actions:

```markdown
## Immediate Next Steps (Complete within 48 hours)

1. [ ] Test deployment pipeline end-to-end in staging environment
2. [ ] Set up monitoring alerts for critical metrics
3. [ ] Create rollback procedures and test them
4. [ ] Validate all environment variables are properly configured

## Short-term (Complete within 1 week)

1. [ ] Implement comprehensive logging for production debugging
2. [ ] Set up automated backup procedures
3. [ ] Create incident response playbook
4. [ ] Conduct load testing with realistic traffic patterns

## Medium-term (Complete within 1 month)

1. [ ] Establish performance benchmarking and alerting
2. [ ] Implement A/B testing framework if applicable
3. [ ] Create comprehensive user documentation
4. [ ] Plan for scaling based on usage growth projections
```

### 9. **Framework Detection Logic**

Auto-detect framework based on:

- **Flutter**: `pubspec.yaml`, `.dart` files
- **React**: `package.json` with React dependencies
- **Vue**: `package.json` with Vue dependencies
- **Node.js**: `package.json` with server dependencies
- **Python**: `requirements.txt`, `pyproject.toml`, `.py` files
- **Go**: `go.mod`, `.go` files

### 10. **Best Practices**

- **Track the checklist** - Record every phase and task in your task-tracking tool with granular detail
- **Breadth AND depth analysis** - Always consider broader architectural implications beyond immediate scope
- **Document decisions comprehensively** - Explain why certain production choices were made with context for future maintainers
- **Proactive questioning** - Always identify what the user should have asked but didn't
- **Test deployment rigorously** - Verify the deployment process works before completion
- **Plan for scale and failure** - Consider what happens when usage grows AND when things break
- **Security first** - Never compromise on security for convenience
- **Always provide actionable next steps** - Give users concrete, prioritized actions to take post-productionization

### 11. **Success Criteria**

The application is production-ready when:

- It handles real-world usage patterns gracefully
- Error scenarios are covered with appropriate user feedback
- Performance is optimized for the target deployment platform
- Configuration can be managed without code changes
- Documentation enables others to deploy and maintain the application
- The deployment process is reliable and repeatable
- Monitoring and observability are in place for production operations
- **Critical questions analysis has been completed** - All important production concerns have been identified and addressed
- **Actionable next steps have been provided** - User has clear, prioritized guidance for post-productionization actions
- **Breadth and depth analysis is complete** - Related systems, architectural implications, and dependencies have been thoroughly considered

## Notes

- Always adapt to the specific framework and deployment target
- Prioritize based on the application's critical user flows and real-world usage patterns
- Focus on production concerns, not development convenience
- Document architectural decisions comprehensively for future maintainers
- Test the complete user journey, not just individual features
- **Always conclude with proactive question analysis and actionable next steps** - This is not optional
- Consider broader architectural implications and related systems impact
