#version 410
layout(std140) uniform;

#define MAX_STEPS 512
#define MIN_DIST 0.5
#define MAX_DIST 200.0
#define EPSILON  0.001

#define MAX_OBJS 5

#define SD_PLANE 0
#define SD_SPHERE 1
#define SD_BOX 2

uniform float iTime;
uniform vec2 iResolution;

uniform mat4 trsf_mat;
uniform mat4 view_mat;
uniform mat4 proj_mat;


in vec3 vColor;


struct Object {      // base aligment    offset
	float size;        // 4                4
	uint  kind;        // 4                8
	uint  material_id; // 4                12
	vec4  pos;         // 16               16
};

uniform Objects { Object objects[MAX_OBJS]; };

struct CastValue {
	float dist;
	uint material_id;
};

struct Material {
	vec3 color;
	float specular;
};



float sdPlane(vec3 p, float h) {
    return p.y - h;
}

float sdSphere(vec3 p, float radius) {
    return length(p) - radius;
}

float sdBox( vec3 p, vec3 b ) {
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float sdTorus( vec3 p, vec2 t ) {
  vec2 q = vec2(length(p.xz)-t.x,p.y);
  return length(q)-t.y;
}

float opSmoothUnion( float d1, float d2, float k ) {
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h); }

CastValue sdScene(vec3 p) {

	
	float dist = 9999999;
	uint material_id = 0;

	for(int i = 0; i < MAX_OBJS; ++i){
		float dist_ = dist;
		switch(objects[i].kind){
			case 0:
				dist_ = sdPlane(p, objects[i].pos.x);
				break;
			case 1:
				dist_ = sdSphere(p - objects[i].pos.xyz, objects[i].size);
				break;
			case 2:
				dist_ = sdBox(p - objects[i].pos.xyz, vec3(objects[i].size));
				break;
		  default:
				break;
		}

		if(dist_ < dist){
				dist = dist_;
				material_id = objects[i].material_id;
		}
	}

	return CastValue(dist, material_id);
}


CastValue rayMarching(vec3 eye, vec3 dir, float start, float end) {
    float depth = start;

		float dist = 0.0;
		uint material_id = 0;
    
    for(int i = 0; i < MAX_STEPS; i++) {
        CastValue cast_value = sdScene(eye + depth * dir);
				dist = cast_value.dist;
        
        if(dist < EPSILON*depth) {
					  material_id = cast_value.material_id;
            return CastValue(depth, material_id) ;
        }
        
        depth += dist;
        
        if(depth >= end){
            return CastValue(end, 0) ;
        }
    }
    
		return CastValue(end, 0) ;
}

vec3 rayDirection(float fov, vec2 size, vec2 fragCoord) {
    vec2 xy = fragCoord - size / 2.0;
    float z = size.y / tan(radians(fov) / 2.0);
    return normalize(vec3(xy, -z));
}

vec3 estimateNormal(vec3 p) {
    return normalize(vec3(
        sdScene(vec3(p.x + EPSILON, p.y, p.z)).dist - sdScene(vec3(p.x - EPSILON, p.y, p.z)).dist,
        sdScene(vec3(p.x, p.y + EPSILON, p.z)).dist - sdScene(vec3(p.x, p.y - EPSILON, p.z)).dist,
        sdScene(vec3(p.x, p.y, p.z + EPSILON)).dist - sdScene(vec3(p.x, p.y, p.z - EPSILON)).dist
    ));
}

float light(vec3 p, vec3 normal, vec3 lightPos) {
    vec3 lightDir = normalize(lightPos - p);
    
    float diff = max(dot(normal, lightDir), 0.0);
    
    return diff;
}

float specular(vec3 p, vec3 normal, vec3 light_pos, vec3 ray_dir) {
	vec3 light_dir = normalize(light_pos - p);
	vec3  hal = normalize(light_dir - ray_dir);
	float dif = clamp( dot( normal, light_dir ), 0.0, 1.0 );
	float spe = pow( clamp( dot( normal, hal ), 0.0, 1.0 ),16.0);
	spe *= dif;
	spe *= 0.04+0.96*pow(clamp(1.0-dot(hal,light_dir),0.0,1.0),5.0);

	return spe;
}

float shadow(vec3 p, vec3 normal, vec3 lightPos) {
    vec3 lightDir = normalize(lightPos - p);
    vec3 newPoint = p + normal*EPSILON;
    
    float k = 32.0;
    float depth = 0.0;
    float ress = 1.0;
    
    for(int i = 0; i < MAX_STEPS; i++) {
        float dist = sdScene(newPoint + depth * lightDir).dist;
        
        if(dist < EPSILON) {
            return 0.0;
        }
        ress = min( ress, k*dist/depth );
        depth += dist;
        
        if(depth >= MAX_DIST){
            return ress;
        }
    }
      
    
    return ress;
    
}

float calcAO( in vec3 pos, in vec3 nor ) {
	float occ = 0.0;
	float sca = 1.0;
	for( int i=0; i<5; i++ )
    {
			float h = 0.01 + 0.12*float(i)/4.0;
			float d = sdScene( pos + h*nor ).dist;
			occ += (h-d)*sca;
			sca *= 0.85;
			if( occ>0.35 ) break;
    }
	return clamp( 1.0 - 3.0*occ, 0.0, 1.0 ) * (0.5+0.5*nor.y);
}

mat4 viewMatrix(vec3 eye, vec3 center, vec3 up) {
	vec3 f = normalize(center - eye);
	vec3 s = normalize(cross(f, up));
	vec3 u = cross(s, f);
	return mat4(
		vec4(s, 0.0),
		vec4(u, 0.0),
		vec4(-f, 0.0),
		vec4(0.0, 0.0, 0.0, 1)
	);
}



void main() {
	Material materials[3];

	materials[0].color = vec3(0.4, 0.99, 0.1);
	materials[1].color = vec3(0.9, 0.1, 0.1);
	materials[2].color = vec3(0.0, 0.0, 0.99);
	
	// relative ray direction
	vec3 dir = rayDirection(75.0, iResolution.xy, gl_FragCoord.xy);
	
	// view mat with no translation
	mat4 invView = inverse(view_mat);
	mat4 viewMatRot = invView;
	viewMatRot[3] = vec4(0.0, 0.0, 0.0, 1.0);

	// observer position
	vec3 eye = invView[3].xyz;
	// absolute ray direction
	vec3 ray_dir = (viewMatRot * vec4(dir, 0.0)).xyz;
	
	// Do raymarching
	CastValue cast_value = rayMarching(eye, ray_dir, MIN_DIST, MAX_DIST);
	// distance between eye and surface
	float dist = cast_value.dist;
	// material of the casted object
	uint material_id = cast_value.material_id;
	
	// position of ligth source
	float time = iTime;
	vec3 lightPos = vec3(sin(time)*10.0, 7.0, cos(time)*10.0);
	
	// final color
	vec3 back_color = vec3(0.0, 0.30, 0.85) + pow((1.0 - 0.99*ray_dir.y), 5);
	vec3 color = back_color;

	// if surface is too far
	if (dist < MAX_DIST - EPSILON) {

		vec3 p = eye + ray_dir*dist;

		vec3 normal = estimateNormal(p);

		// Output to screen

		vec3 mate = vec3(0.2);
		vec3 base_color = materials[material_id].color;
		vec3 sun_color = vec3(0.9, 0.85, 0.6);
		vec3 bounce_color = vec3(0.04, 0.04, 0.04);
		vec3 sky_color = vec3(0.1, 0.35, 0.8);

		float sun_force = 15.0;
		float sky_force = 0.3;
		float bounce_force = 1.0;

		float sun_diff = light(p, normal, lightPos ) * sun_force;
		float spec_diff = specular(p, normal, lightPos, ray_dir);
		float sky_diff = clamp( 0.5 + 0.5*dot(normal, vec3(0.0, 1.0, 0.0)), 0.0, 1.0) * sky_force;
		float shad_diff = shadow(p, normal, lightPos );
		float bounce_diff = clamp( 0.5 + 0.5*dot(normal, vec3(0.0, -1.0, 0.0)), 0.0, 1.0) * bounce_force;
		float ao_diff = calcAO(p, normal);

		vec3 color_penumbra = pow(vec3(shad_diff), vec3(1.0, 1.4, 1.9));
		color = mate*base_color * sun_color * sun_diff * color_penumbra;
		//color += spec_diff;
		color += sky_color * sky_diff;
		color += bounce_color * bounce_diff;
		color *= ao_diff;
	}

	// Gamma correction
	//color = pow(color, vec3(0.4545));
	// mist
	//color = mix(color, back_color, 1.0 - exp(-0.0000001*dist*dist*dist*dist));
	color = mix(color, back_color, vec3(smoothstep(120.0, 150.0, dist)));
	color = vec3(objects[2].pos.xyz);

	gl_FragColor = vec4(color,1.0);
}
