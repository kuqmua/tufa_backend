use std::collections::HashMap;
pub fn get_twitter_links() -> HashMap<&'static str, &'static str> {
    let twitter_sections_links: HashMap<&str, &str> = [
        ("physorg_com", "https://nitter.42l.fr/physorg_com/rss"),
        ("joebarnard", "https://nitter.42l.fr/joebarnard/rss"),
        OK9UWU,
        r2x0t,
        Katha16777,
        PataniLab,
        AlexSerbul,
        TheCherno,
        rustlinz,
        HopeMarsMission,
        dougbinks,
        remodemo,
        italiancpp,
        ilpropheta,
        alepezzato,
        rstropek,
        agrimgupta92,
        GraphicMeetup,
        ShorterLab,
        werf_io,
        Back2Warcraft,
        AvoydGame,
        habr_eng,
        AnalysisSensing,
        abhi_tweeter,
        ozkriff_ru,
        KudSverchkov,
        AstonlabsPurdue,
        kermitmurray,
        OatesLab,
        GeiselBiofilm,
        GamedevStefan,
        ghodges_dev,
        JonathanGMannLD,
        iFiery,
        _jeck,
        krajzeg,
        scottstoll2017,
        WhiteDo27114277,
        vector_of_bool,
        Suthar3Aryan,
        flutter_jobs,
        Jiayin_Cao,
        ScopeShifu,
        ApoorvaJ,
        vexastrae,
        Stefan_W_Hell,
        FlutterWarsaw,
        BocaChicaGal,
        MiSvTh,
        ITSURYUU,
        kushirosea,
        WataruVFX,
        Prokaryota,
        QiaochuYuan,
        ToughSf,
        flight404,
        luwizart,
        angew_chem,
        KnativeProject,
        ulalaunch,
        ChevryMarc,
        guillain_barre,
        AJR_Radiology,
        van__Oijen,
        fasterthanlime,
        mattatz,
        Rocha_Lab,
        zzznah,
        jmutterer,
        Tuatara_VFX,
        Wardl_,
        mobileunderhood,
        doescience,
        raisingsun6665,
        PeterFabianLab,
        BioRender,
        DarArvin,
        podcast_znprod,
        _willfalcon,
        Remesher_,
        ndee85,
        RubenR3D,
        Adrien_nayrat,
        ThePracticalDev,
        polycount,
        kendrickszb,
        ZebrafishRock,
        A_blender_user,
        UnderJS,
        JesseMiettinen,
        KunosStefano,
        RustFest,
        RustConAsia,
        MingshenSun,
        BaiduXlab,
        NatureComms,
        InfographicTony,
        Sanctus_Art,
        jongranskog,
        bagder,
        dimforge,
        carlosedubarret,
        burntsushi5,
        eigensteve,
        dieselframework,
        CliRust,
        lucio_d_franco,
        ShekoHex,
        ShekoHex
        thekbknapp,
        matthiasendler,
        hellorustshow,
        ryan_levick,
        jntrnr,
        vengarioth,
        DPC_22,
        Erstejahre,
        killercup,
        robert_winslow,
        wvo,
        yaahc_,
        Carter_Lab,
        TokamakUI,
        kebabskal,
        Sakura_Rabbiter,
        MichalLytek,
        sagzehn,
        S_LevequeFort,
        The_ArtOfCode,
        awwbees,
        marcel_campen,
        lateasusual_,
        attiegrande,
        pcwalton,
        wittyelk,
        FlutterFireDev,
        kar_sourav,
        Gabriel_Risa,
        BlenderBIM,
        Renato3xl,
        ChenHuang96,
        RaizNewMedia,
        NASAGoddard,
        NASASolarSystem,
        NASAKennedy,
        NASA_Technology,
        nasahqphoto,
        NASA_Astronauts,
        AsteroidWatch,
        medrxivpreprint,
        ReplicabilityG,
        playcanvas,
        AndreaBassi78,
        StereoVinny,
        TidalFlask,
        FengZhuDesign,
        biorxiv_sysbio,
        biorxiv_neursci,
        biorxiv_micrbio,
        biorxiv_genetic,
        biorxiv_evobio,
        biorxiv_ecology,
        biorxivpreprint,
        biorxiv_genomic,
        biorxiv_bioinfo,
        entagma,
        eddbiddulph,
        SoluSerg,
        JamesBlasco,
        BlenderNPR,
        vlusky_husky,
        val_sagrario,
        Abdelfattah__A,
        vprobon,
        joshuamdeguzman,
        flutter_exp,
        letsbuildgg,
        panzerdp,
        _Korchiy_,
        TheGregYang,
        D2L_ai,
        Natzke,
        simonstoschu,
        Underfox3,
        Index_3D,
        benprudhomme1,
        shanenewville,
        brianglancylab,
        Astro_Doug,
        AstroBehnken,
        vr_sebas,
        Leukbaars,
        4DNucleome,
        petervandervel3,
        DunsingValentin,
        stephen_bester,
        scita_lab,
        TheYapLab,
        eriksahailab,
        ActonLab,
        Kaplanyan,
        BCiechanowski,
        OlexaLe,
        JiYiLight,
        cgmastersnet,
        cg_geeks,
        Bbbn192,
        derbender4,
        StashVertex,
        NDunes3D,
        boxvfx,
        karll_henning,
        filedescriptors,
        vcubingx,
        ZanQdo,
        PaoloFurani,
        LabSauer,
        FerrousSystems,
        rust_analyzer,
        rust_gamedev,
        munlangorg,
        AmethystEngine,
        memprotmd,
        lesloew,
        BiophysJ,
        biorxiv_biophys,
        PlantPhys,
        azonenberg,
        johan_elf_,
        kitasenjudesign,
        Jan_de_Vries,
        JXBot,
        bryotweet,
        JacquesLucke,
        NASAPersevere,
        PThuriot,
        AlphaWhiskey82,
        CBI_Pitt,
        YangLiuLab1,
        Geeks3D,
        BrookhavenLab,
        Fermilab,
        alksndrkili,
        Vochsel,
        sai_charan_md,
        WinnichenkoD,
        LesleyLai6,
        VGVentures,
        SuprDeclarative,
        cgonfire,
        luigifcruz,
        agriclaudia,
        keyframes_tw,
        nikomatsakis,
        ServoDev,
        rustwasm,
        read_rust,
        rustaceanfm,
        jonhoo,
        4minus1d,
        CorneliusGati,
        NucleusSciTalks,
        QianPeterSu,
        phantom_owl,
        serhii_rieznik,
        FritzscheLab,
        abbelight,
        GoncaloFDS,
        RandomBlendings,
        KloudStrife,
        dchaplot,
        oxcsml,
        jinxu06,
        kasparmartens,
        emidup,
        MarcusHilsdorf,
        martin_gorner,
        arXiv_Daily,
        a13xp0p0v,
        rajammanabrolu,
        SeptinLab,
        daniela_barilla,
        sqaunderhood,
        TychoBolt,
        SirWadeFX,
        SebDeguy,
        natBME,
        pavmike,
        im_galad,
        ZoeyFan723,
        ShechtmanLab,
        eric_heitz,
        torch_in_sky,
        fpelisch,
        ritafior,
        agvanimation,
        kD3AN,
        guiwitz,
        miguelbandera,
        PappulabWashU,
        GregMadison,
        ebarranko,
        danielsantalla,
        fayezsalka,
        ilya_aparin,
        NaturallyCG,
        OrestesGaolin,
        erindale_xyz,
        chippwalters,
        Raspberry_Pi,
        ykilcher,
        josmil1,
        rileyb3d,
        asahidari,
        revoider,
        blacksquirrel__,
        GuillaumeLample,
        belle2collab,
        basit_ayantunde,
        dwrensha,
        Andi_Microscopy,
        RupeshMandke,
        rustbeltrust,
        RustVideos,
        rustconf,
        newrustacean,
        ThisWeekInRust,
        zombodb,
        svartalf,
        tokio_rs,
        RustTrending,
        RustLibHunt,
        haraldhoyer,
        246R_Bloomin,
        timsneath,
        J_A_C_S,
        BettiniGabe,
        IlyaKuzovkin,
        pgexperts,
        2ndQuad,
        PostgresWeekly,
        Cuboxel,
        maxSigma_,
        ArtFromRachel,
        RanaHanocka,
        yoongs,
        CrossmindStudio,
        danielepolencic,
        phosphoer,
        jobtalle,
        wiersma_ruben,
        SilenceMoon_Yue,
        linolafett,
        Froyok,
        exppad,
        IndiaFlutter,
        DevenJoshi7,
        erikskog_,
        Deep__AI,
        EuropHospital,
        rustlang,
        JoshWComeau,
        deno_land,
        Icare3D,
        tunabrain,
        Stubbesaurus,
        tristanbrindle,
        NiloCat_Colin,
        Oscurart,
        Atrix256,
        nixcraft,
        Mirko_Salm,
        FlutterLDN,
        FlutterReleases,
        geekmz,
        CGuivant,
        felangelov,
        artofjeffp,
        brendandjcad,
        Vasyl72728301,
        simurai,
        clayxels,
        VisualStudio,
        imthepk,
        phi_lira,
        MiegakureGame,
        PaulC04,
        boksajak,
        stevestreeting,
        joachimgoedhart,
        Cabbibo,
        london_lab,
        games_inu,
        m_schuetz,
        PhysicalAddons,
        larry73451236,
        portnov9,
        AnatoleDuprat,
        MartinStich,
        0xca0a,
        rianflo,
        John_O_Really,
        arecaplm,
        crascit,
        CERN,
        neilt3d,
        sylefeb,
        _TomekS,
        BartWronsk,
        TheAllenChou,
        3drwny,
        suishess,
        xbresson,
        harrietm11,
        biggsneal1,
        SpectreSkully,
        matthen2,
        fael097,
        MacchiatoLycan,
        juulcat,
        artistcdmj,
        mikulasflorek,
        colejefferies,
        nelstuff,
        chriskwallis,
        ispc_updates,
        noazark,
        SenguptaUmd,
        KevinCadieuxMS,
        ejdeon,
        HCI_Research,
        kenshirriff,
        mraleph,
        JS_Cheerleader,
        adampi,
        LaineBioImaging,
        notargs,
        DavideCalebiro,
        BioGridGame,
        stelabouras,
        hekiba_io,
        MasterWhet,
        vitos1k,
        BrianLeleux,
        FAGIOLOVOLANTE,
        keenanisalive,
        victorvdr9,
        jeffamstutz,
        quasimondo,
        GoogleAI,
        AravSrinivas,
        uschmidt83,
        jaguring1,
        louisdumont,
        happy_modeling,
        AliceInNanoland,
        WWeynants,
        godsgreg,
        dedouze_,
        nik_vili,
        AlainChedotal,
        var_bincom,
        romainguy,
        MattNiessner,
        ewers_helge,
        PixelClearr,
        Inasa_Fujio,
        PlatypusAdvent1,
        fredric_,
        luamono,
        VicDoval,
        ProdeusGame,
        llazyadM,
        Carlillo,
        Donzanoid,
        pixnblox,
        stefanzellmann,
        Wayward_Art_Co,
        mluparu,
        johnparsaie,
        panlepan,
        dfranx_,
        flutterinst,
        flutterdevs,
        FlutterWk,
        lipmanya,
        NASA_SLS,
        openspim,
        acmarr,
        KSpaceAcademy,
        PromPreprint,
        AurelioReis,
        neilbickford,
        MaxPuliero,
        hollasch,
        CodeerDev,
        FakhriLab,
        chrisprenn,
        __xuorig__,
        SongForHumanity,
        erikswahn,
        DannyBittman,
        schneckerstein,
        Boycraf19492179,
        javarevisited,
        Azadux,
        GradManuel,
        TylerG1998,
        hsalis,
        inkasipola,
        syoyo,
        kotsoft,
        ISS_Research,
        compoundchem,
        gp_pulipaka,
        sulco,
        martin_maas,
        ourmachinery,
        jrpowers,
        kyXtak,
        ProfTomEllis,
        C_M_Hobson,
        flutterize,
        UriGoldshtein,
        FullstackDevJS,
        NECKOPN,
        Feyris77,
        norbertkozsir,
        jhorikawa_err,
        CodyWinch,
        ChicagoFlutter,
        FlutterEurope,
        _eseidel,
        redbrogdon,
        filiphracek,
        soragnilab,
        jlizier,
        zipcpu,
        NOTLonely92,
        UE4Memes,
        Nosferalatu,
        djowel,
        PeterWBattaglia,
        BaranLabReads,
        moxstudios,
        Robert12415877,
        davephoffman,
        v6_prime,
        tvaneerd,
        JesseBrophy,
        jerbotnet,
        3blue1brown,
        Astro_Jessica,
        johannaivaska,
        anna_medyukhina,
        jonathangrimm,
        aras_p,
        MagnusL3D,
        BlenderDiplom,
        cinedatabase,
        rita_strack,
        OlixOliver,
        hey_michaelh,
        kevin_tsia,
        gamozolabs,
        pixelmager,
        JanWillemTulp,
        ddiakopoulos,
        SasaBudimir,
        medical_xpress,
        yaroslav_ganin,
        charlietcnash,
        cobalt_kura,
        Koola_UE4,
        SignalCambridge,
        moyix,
        mpicbg,
        diffusiveblob,
        ankurhandos,
        HohlbeinLab,
        FredTingaudDev,
        NaturePhysics,
        physorg_com,
        WendeNGibbs,
        ambrosiussen_p,
        CoolSpotDreamer,
        maartenjhof,
        ma1andrea,
        anatudor,
        MartinNebelong,
        Addgene,
        DennysKuhnert,
        wilnyl,
        alexis_gil,
        FaustXins,
        simon_stix,
        erturklab,
        jaaanaru,
        flutter_school,
        AndrePulschen,
        BrianKelch,
        alexanderbelyy1,
        myosinactncrazy,
        benoitbruneau,
        HubbleTelescope,
        EvanHallMD,
        sebmck,
        DrSalazarMejia,
        FabioSchutz78,
        DrNikhilJoshiMD,
        DrOmarMian,
        ShaanDudani,
        neerajaiims,
        QueckOlli,
        MatNiedoba,
        ferristweetsnow,
        FlutterBlogs,
        freeman_as,
        pepijndevos,
        horse_js,
        MyNameIsMJP,
        ScienceTM,
        elmanmansimov,
        inresin,
        __simt__,
        3Dmattias,
        NatRevImmunol,
        NatRevClinOnco,
        BennyGovaerts,
        takase_s,
        kytwb,
        jsoriamd,
        GowthamRaj100,
        katsuya_tsukui,
        Booster_Buddies,
        AndroidDev,
        IBMDeveloper,
        Dr_RaviMadan,
        preshing,
        zaidalyafeai,
        kimitalvitie,
        ShinpeiKato,
        11thDream_Game,
        kozzzlove,
        shaderpunk_jp,
        sedrewed,
        ico_TC,
        slurpsmadrips,
        biz84,
        SebastianLague,
        flutterdevcast,
        codemagicio,
        d_gfx,
        DartHype,
        DartCode,
        FlutterComm,
        Flutter_Flakes,
        flutteriodaily,
        dart_lang,
        ShuregDenisov,
        mrdoob,
        FerrenceG,
        AssemblyScript,
        torch2424,
        FirefoxDevTools,
        mhadaily,
        amuellerml,
        _ramoreira,
        IntelSecurity,
        scott_e_reed,
        marsrader,
        Smerity,
        three_cube,
        FullStackFest,
        soshace_com,
        AlinaShumarina,
        strewnify,
        NASARoman,
        Aidan_Wolf,
        spornslab,
        GregScott_photo,
        deanwampler,
        TimSweeneyEpic,
        Inoryy,
        mike1pol,
        tobiasmarciszko,
        greggman,
        myshov,
        LuciusFekonja,
        miloseviclab,
        MadelonMaurice,
        AndreySozykin,
        MatthewDean3D,
        cman2k,
        Dr_Ivanoncologo,
        CNC_Kitchen,
        arcanis,
        rezoner,
        slashML,
        TaroOzaki,
        seanbax,
        Firebase,
        _davideast,
        BP_Hutch,
        a00rs,
        MarescaLab,
        NetanelBasal,
        wysscoray,
        nori_shinoda,
        ChemPlusChem,
        strudlzrout,
        glukozica,
        fraser_lab,
        marian42_,
        MarcelAMller,
        KLEIJ_ICIQ,
        ECycles1,
        MolinaGroup,
        Cyanilux,
        nestframework,
        N_Tepluhina,
        eems_mit,
        sitnikcode,
        florianjug,
        benawad,
        EmporiumThought,
        ClassyDogFilms,
        sculptjanuary,
        lunasorcery,
        mattstark256,
        ICCV19,
        GitNationOrg,
        mclow,
        LeiTian14,
        GolangKazan,
        AterCattus,
        behindthescope_,
        Johannes_Karges,
        ntziachristos,
        rsalakhu,
        rauchg,
        vercel,
        FrederickWelt,
        leondenise,
        nizzle_fe_shizz,
        NVIDIAEmbedded,
        NVIDIAHPCDev,
        NVIDIADC,
        NVIDIAGameDev,
        david_obrien,
        KKyrimis,
        US_SpaceCom,
        JimBridenstine,
        keen_tools,
        Aiims1742,
        salkinstitute,
        MrB_Jensen,
        ProgrammerLin,
        TheHackersNews,
        zetuZT,
        denisivanov,
        MicroArchConf,
        sigarch,
        TitusWinters,
        harrism,
        LambdaConcept,
        SAFARI_ETH_CMU,
        _onurmutlu_,
        cppedinburgh,
        nicoscherf,
        Astro_Ferg,
        massimorighi,
        prince_xml,
        KAvignon,
        NEUBIAS_COST,
        UCL_IPLS,
        JPMajor,
        _LaszloZoltan_,
        Simon_Houdini,
        ProjectJupyter,
        fchollet,
        DrCEWilloughby,
        InstLatX64,
        dnesteruk,
        dendibakh,
        csssr_dev,
        ThompsonLab,
        JCellBiol,
        Dev_Bio_Journal,
        DevelopmentalDy,
        bxv_dev,
        aslushnikov,
        MattMirrorFish,
        3Lateral,
        thinkmariya,
        Sam_Makes_Games,
        mattmiesnieks,
        g33konaut,
        murosyoukei,
        ASCBiology,
        cshperspectives,
        detective_horse,
        robinmanuelt,
        jgsogo,
        careyjans,
        NicoJosuttis,
        Icy_BioImaging,
        BioImagingUK,
        roshan_m_rao,
        bgolus,
        stoyanstefanov,
        brian_lovin,
        rochaandreal,
        LaissuePhilippe,
        NikonSmallWorld,
        JustinPaver,
        RReverser.
        wasmerio,
        GeneralistP,
        2112_games,
        WindowsDocs,
        lucengame,
        ZaedDB,
        RebeccaRHelm,
        NathanShaner,
        FiolkaLab,
        RetoPaul,
        KerenGu,
        GoodAIdev,
        DanfengCai,
        andrey_akinshin,
        the_f_key,
        _BD3D,
        forwebdev,
        jeffbarr,
        GMFHx,
        d_brueckner,
        DougPShepherd,
        jengreitz,
        AcceleratorNick,
        MaciejTreder,
        damiandn,
        CharlotteFare,
        BlackHC,
        DesignSpark_JP,
        DonaldM38768041,
        aire_team,
        BateupLab,
        alt_kia,
        marcan42,
        luizkruel,
        obenjiro,
        lradoshevich26,
        SedonaMurphy,
        vanderlin,
        ovrweb,
        TrackingActions,
        KshitizKz,
        posva,
        EmmanuelMagnier,
        KyongFAM,
        srush_nlp,
        mlfrg,
        ManuelaXibanya,
        mattersOfLight,
        AlexKontorovich,
        LawdOdin,
        skx_doom,
        trusty_games,
        mxsage,
        grpcio,
        Tankred_Daeron,
        pitercss_meetup,
        JasperRLZ,
        longbool,
        BartekMoniewski,
        3d_eric,
        rage_monk,
        bunopus_en,
        AJ_FI,
        amel_true,
        mohitban47,
        jaseweston,
        parlai_parley,
        ensou_art,
        VitaliyKirenkov,
        xenophar,
        ppaawweeuu,
        grgrdvrt,
        FranziskaPanter,
        Blackrabbit99,
        kottans_org,
        xanf_ua,
        miziziziz,
        shwars,
        NathanGDQuest,
        godotengine,
        _AmazingAssets,
        trav_downs,
        pierrci,
        _alexeykalinin,
        lightarchitect_,
        evilmartians_ru,
        SaiyamPathak,
        vitessio,
        PostCSS,
        kotlin,
        archaeal,
        _KudoHiroyuki,
        jaredpalmer,
        _benleblanc,
        johntwolives,
        biorxiv_cellbio,
        Vitalliumm,
        hamiltonulmer,
        LevelPixelLevel,
        38912_DIGITAL,
        remi_creative,
        arborillustrate,
        profbof,
        h3r2tic,
        polygonrunway,
        izutionix,
        _poei,
        quixeltools,
        argyleink,
        123gas321,
        pky_blog,
        joaomoreno,
        james_madhacks,
        KaverinaLab,
        LucaRood,
        yarpoplar,
        peterekepeter,
        HolyJSconf,
        alfcnz,
        memecrashes,
        MarcoDiVita6,
        TF_siri,
        NateMorrical,
        NanoLiveLtd,
        ____lighthouse,
        aerotwist,
        mathias,
        jaffathecake,
        DasSurma,
        LightUpScience,
        Sentdex,
        DotNextConf,
        nevalau,
        CEITEC_Brno,
        ozmant,
        nathanduck88,
        mikegarndesign,
        johannes_wilde,
        The_BenMears,
        museumofcomm,
        AboticsG,
        orestiskon,
        KochkinGames,
        kengoito1110,
        RubenEVillegas,
        siddharthkp,
        Pawige,
        t_looman,
        DiegoSaraiva,
        mjhigley,
        profLiangGao,
        GraceIHsu,
        HansClevers,
        neurosocialself,
        kammysliwiec,
        QueerJS,
        hardsci,
        twi_mar,
        PolystreamE,
        ChengXianrui,
        _jdevos_,
        NikolayOskolkov,
        abeysaurus,
        skaven_,
        babylonjs,
        ThomasKoleTA,
        maisam_hosaini,
        BayuItra,
        fletchgraham,
        JayMBroderick,
        JoeyDeVriez,
        70_cg_art,
        devoopsconf,
        rickyreusser,
        reduzio,
        graphqlsummit,
        BigDataToolsJB,
        AndrewGYork,
        lopoisaac,
    ]
    .iter()
    .cloned()
    .collect();
    twitter_sections_links
}
