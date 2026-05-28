import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8000';

let authToken: string;

test.describe('Items and Actions', () => {
  test.beforeEach(async ({ request }) => {
    // Register and login
    const email = `test-${Date.now()}@example.com`;

    await request.post(`${BASE_URL}/api/auth/register`, {
      data: {
        email,
        name: 'Adventurer',
        password: 'password123',
      },
    });

    const loginResponse = await request.post(`${BASE_URL}/api/auth/login`, {
      data: { email, password: 'password123' },
    });

    const loginData = await loginResponse.json();
    authToken = loginData.data.token;
  });

  test('should list items in current area', async ({ request }) => {
    const listResponse = await request.get(`${BASE_URL}/api/areas`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    const areas = await listResponse.json();
    const areaId = areas.data.areas[0].id;

    // Enter area
    await request.post(`${BASE_URL}/api/areas/${areaId}/enter`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    // Get items
    const response = await request.get(`${BASE_URL}/api/areas/${areaId}/items`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(Array.isArray(data.data.items)).toBe(true);
  });

  test('should collect item from area', async ({ request }) => {
    const listResponse = await request.get(`${BASE_URL}/api/areas`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    const areas = await listResponse.json();
    const areaId = areas.data.areas[0].id;

    // Enter area
    await request.post(`${BASE_URL}/api/areas/${areaId}/enter`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    // Get items
    const itemsResponse = await request.get(`${BASE_URL}/api/areas/${areaId}/items`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    const items = await itemsResponse.json();

    if (items.data.items.length > 0) {
      const itemId = items.data.items[0].id;

      // Collect item
      const response = await request.post(
        `${BASE_URL}/api/areas/${areaId}/items/${itemId}/take`,
        {
          headers: { Authorization: `Bearer ${authToken}` },
        }
      );

      expect(response.status()).toBe(200);
      const data = await response.json();
      expect(data.status).toBe('ok');
    }
  });

  test('should fail to collect item without capacity', async ({ request }) => {
    // This test requires creating a scenario where inventory is full
    // TODO: Fill inventory with items
    // TODO: Try to collect one more
    // TODO: Verify it fails with capacity error
  });

  test('should perform base action', async ({ request }) => {
    const listResponse = await request.get(`${BASE_URL}/api/areas`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    const areas = await listResponse.json();
    const areaId = areas.data.areas[0].id;

    // Enter area
    await request.post(`${BASE_URL}/api/areas/${areaId}/enter`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    // Perform action
    const response = await request.post(
      `${BASE_URL}/api/areas/${areaId}/actions`,
      {
        data: { action: 'examine' },
        headers: { Authorization: `Bearer ${authToken}` },
      }
    );

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(data.data.outcome).toBeTruthy();
  });

  test('should perform action requiring item', async ({ request }) => {
    // TODO: Get area with locked chest
    // TODO: Try to open without key - should fail
    // TODO: Collect key from area
    // TODO: Open with key - should succeed
  });

  test('should track item quality degradation', async ({ request }) => {
    // TODO: Collect item with quality tracking
    // TODO: Use item
    // TODO: Get item details
    // TODO: Verify quality decreased
  });

  test('should track item durability', async ({ request }) => {
    // TODO: Collect item with durability
    // TODO: Use item multiple times
    // TODO: Verify durability decreased
    // TODO: Item breaks when durability reaches 0
  });

  test('should repair broken item', async ({ request }) => {
    // TODO: Break an item
    // TODO: Collect repair tools
    // TODO: Repair item
    // TODO: Verify item works again
  });

  test('should view event log of actions', async ({ request }) => {
    // Perform some actions
    const listResponse = await request.get(`${BASE_URL}/api/areas`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    const areas = await listResponse.json();
    const areaId = areas.data.areas[0].id;

    await request.post(`${BASE_URL}/api/areas/${areaId}/enter`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    // Get event log
    const response = await request.get(`${BASE_URL}/api/player/events`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(Array.isArray(data.data.events)).toBe(true);
  });
});
