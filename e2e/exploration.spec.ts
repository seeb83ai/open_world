import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8000';

let authToken: string;
let userId: string;

test.describe('Exploration Flow', () => {
  test.beforeEach(async ({ request }) => {
    // Register and login to get token
    const email = `test-${Date.now()}@example.com`;

    await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        email,
        name: 'Explorer',
        password: 'password123',
      },
    });

    const loginResponse = await request.post(`${BASE_URL}/api/auth/login`, {
      data: { email, password: 'password123' },
    });

    const loginData = await loginResponse.json();
    authToken = loginData.data.token;
    userId = loginData.data.user.id;
  });

  test('should get list of areas', async ({ request }) => {
    const response = await request.get(`${BASE_URL}/api/areas`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(Array.isArray(data.data.areas)).toBe(true);
    expect(data.data.areas.length).toBeGreaterThan(0);
  });

  test('should get specific area details', async ({ request }) => {
    // Get list of areas first
    const listResponse = await request.get(`${BASE_URL}/api/areas`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    const areas = await listResponse.json();
    const areaId = areas.data.areas[0].id;

    // Get specific area
    const response = await request.get(`${BASE_URL}/api/areas/${areaId}`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(data.data.area.id).toBe(areaId);
  });

  test('should enter area and see current state', async ({ request }) => {
    const listResponse = await request.get(`${BASE_URL}/api/areas`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    const areas = await listResponse.json();
    const areaId = areas.data.areas[0].id;

    const response = await request.post(`${BASE_URL}/api/areas/${areaId}/enter`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(data.data.area.id).toBe(areaId);
  });

  test('should fail to access area without authentication', async ({ request }) => {
    const response = await request.get(`${BASE_URL}/api/areas`);

    expect(response.status()).toBe(401);
    const data = await response.json();
    expect(data.code).toBe('UNAUTHORIZED');
  });

  test('should track player position', async ({ request }) => {
    // Get player position
    const response = await request.get(`${BASE_URL}/api/player/position`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(data.data.position).toBeTruthy();
  });

  test('should list player inventory', async ({ request }) => {
    const response = await request.get(`${BASE_URL}/api/player/inventory`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(Array.isArray(data.data.inventory)).toBe(true);
  });
});
