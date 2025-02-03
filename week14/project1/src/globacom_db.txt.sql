--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    cid integer NOT NULL,
    cname text NOT NULL,
    c_age integer NOT NULL,
    c_email character varying(30) NOT NULL,
    cmobile character(15) NOT NULL,
    e_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: data_plan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.data_plan (
    data_id integer NOT NULL,
    data_size character varying(7) NOT NULL,
    dataduration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.data_plan OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration character varying(10) NOT NULL,
    pmanagerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    staff_sal real NOT NULL,
    age integer NOT NULL,
    mobile character(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (cid, cname, c_age, c_email, cmobile, e_id, data_id) FROM stdin;
110	Musta Karim	32	m_karim@gmail.com	08055089112    	102	5
111	Lillian Jaiya	43	l_jaite@gmail.com	08055185341    	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813    	107	10
113	Phillip Akonjo	41	p_akonjo@gmail.com	09052356772    	100	2
114	Marylene Napa	33	m_mapa@gmail.com	08053333551    	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774    	117	11
116	Adams Bree	33	a_bree@gmail.com	07056763367    	102	1
117	Okofar Matthais	45	o_matthias@gmail.com	08056763357    	120	10
118	Samson Adeleke	65	s_adaleke@gmail.com	07056774423    	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	 09052111101   	107	5
120	James Bob	44	j_bob@gmail.com	08059692919    	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144    	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923    	107	5
\.


--
-- Data for Name: data_plan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.data_plan (data_id, data_size, dataduration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, pmanagerid) FROM stdin;
11	A	9 months	102
22	B	14 MONTHS	97
33	C	16 MONTHS	120
44	D	25 MONTHS	108
55	E	9 MONTHS	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
101	Alade Joy	2	250000	33	08023089832    
100	Mustapha Ali	3	175000	32	08063285831    
107	Alokwe Martin	7	380000	48	07090082812    
97	Dankade Aminat	5	550000	40	09023688832    
102	Makinde Mary	2	450000	55	09023487830    
120	Adeleke Jane	4	200000	38	07061045862    
122	Osahon Mark	6	320000	44	08022289842    
117	Suleman Ajayi	3	800000	50	07030089811    
104	Kuti Lawal	1	750000	35	09145689842    
108	Josiah Joshua	1	120000	30	08053189135    
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (cid);


--
-- Name: data_plan data_plan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.data_plan
    ADD CONSTRAINT data_plan_pkey PRIMARY KEY (data_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- Name: staff staff_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT staff_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

