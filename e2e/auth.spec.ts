import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8000';

test.describe('Authentication Flow', () => {
  test('should register a new user', async ({ request }) => {
    const response = await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        email: `test-${Date.now()}@example.com`,
        name: 'Test User',
        password: 'secure_password_123',
      },
    });

    expect(response.status()).toBe(201);
    const data = await response.json();
    expect(data.status).toBe('ok');
  });

  test('should reject registration with missing email', async ({ request }) => {
    const response = await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        name: 'Test User',
        password: 'secure_password_123',
      },
    });

    expect(response.status()).toBe(400);
    const data = await response.json();
    expect(data.code).toBe('VALIDATION_ERROR');
  });

  test('should reject duplicate email registration', async ({ request }) => {
    const email = `test-${Date.now()}@example.com`;

    // First registration
    await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        email,
        name: 'Test User',
        password: 'secure_password_123',
      },
    });

    // Second registration with same email
    const response = await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        email,
        name: 'Another User',
        password: 'another_password',
      },
    });

    expect(response.status()).toBe(409);
    const data = await response.json();
    expect(data.code).toBe('CONFLICT');
  });

  test('should login with valid credentials', async ({ request }) => {
    const email = `test-${Date.now()}@example.com`;

    // Register user
    await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        email,
        name: 'Test User',
        password: 'secure_password_123',
      },
    });

    // Login
    const response = await request.post(`${BASE_URL}/api/auth/login`, {
      data: {
        email,
        password: 'secure_password_123',
      },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(data.data.token).toBeTruthy();
    expect(data.data.user.email).toBe(email);
  });

  test('should reject login with wrong password', async ({ request }) => {
    const email = `test-${Date.now()}@example.com`;

    // Register user
    await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        email,
        name: 'Test User',
        password: 'secure_password_123',
      },
    });

    // Try to login with wrong password
    const response = await request.post(`${BASE_URL}/api/auth/login`, {
      data: {
        email,
        password: 'wrong_password',
      },
    });

    expect(response.status()).toBe(401);
    const data = await response.json();
    expect(data.code).toBe('UNAUTHORIZED');
  });

  test('should reject login with non-existent user', async ({ request }) => {
    const response = await request.post(`${BASE_URL}/api/auth/login`, {
      data: {
        email: 'nonexistent@example.com',
        password: 'any_password',
      },
    });

    expect(response.status()).toBe(401);
  });
});
