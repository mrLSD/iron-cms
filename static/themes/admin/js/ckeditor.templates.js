CKEDITOR.addTemplates('sliders.old',
    {
        templates :
            [
                {
                    title: 'Template 1',
                    description: 'Шаблон для слайдера 1',
                    html: '<div class="content-wrap theme1"> ' +
                    '<div class="content-text"> ' +
                    '<h1>Лучшие учителя Йоги</h1> ' +
                    '<h2>Постоянно совершенствуйтесь, занимаясь у лучших преподавателей!</h2> ' +
                    '{{isGuest::<p class="text"> <a class="orange-btn fancybox" href="#sign-up"><span>Начать Занятия</span></a> <a class="promovideo-btn fancybox promovideo" href="#promovideo">Узнать больше</a> </p> ' +
                    '<p class="text"> Зарегистрируйтесь и получите неделю в подарок! </p> }}' +
                    '</div> </div>'
                },
                {
                    title: 'Template 2',
                    description: 'Шаблон для слайдера 2',
                    html: '<div class="content-wrap theme2"> ' +
                    '<div class="content-text"> ' +
                    '<h1><span>Первая</span> Онлайн Школа Йоги </h1> ' +
                    '<h2>с ведущими преподавателями для начинающих и опытных йогов</h2> ' +
                    '{{isGuest::<p class="text"> <a class="orange-btn fancybox" href="#sign-up"><span>Начать Занятия</span></a> <a class="promovideo-btn fancybox promovideo" href="#promovideo">Узнать больше</a> </p> ' +
                    '<p class="text"> Зарегистрируйтесь и получите неделю в подарок! </p> }}' +
                    '</div> </div>'
                },
                {
                    title: 'Template 3',
                    description: 'Шаблон для слайдера 3',
                    html: '<div class="content-wrap theme3"> ' +
                    '<div class="content-text"> ' +
                    '<h1>Везде  и всегда!</h1> ' +
                    '<h2>Занимайся йогой в любом <br /> месте и любом устройстве</h2> ' +
                    '{{isGuest::<p class="text"> <a class="orange-btn fancybox" href="#sign-up"><span>начни сейчас</span></a></p> ' +
                    '<p class="text">Зарегистрируйтесь и получите неделю в подарок! </p> }}' +
                    '</div> ' +
                    '<img src="/themes/daYoga/images/device.png" alt="image description" /> ' +
                    '<a class="promovideo-play fancybox {{isGuest:promovideo}}" href="{{isGuest::#promovideo::/vse_video}}">Узнать больше</a> </div>'
                }
            ]
    }
);

CKEDITOR.addTemplates('sliders',
    {
        templates :
            [
                {
                    title: 'Нижний блок с регистрацией',
                    description: '',
                    html: '<!-- sidebar --> ' +
                    '<div class="slider-sidebar"> ' +
                    '{{isGuest::<div class="container"> ' +
                    '<div class="slider-sidebar__offer-registration"> ' +
                    '<a class="orange-btn fancybox" href="#sign-up">НАЧНИ СЕЙЧАС</a>' +
                    '<div class="slider-sidebar__offer-registration-info">зарегистрируйся<br> и получи <span class="selected">неделю в подарок</span></div>' +
                    '</div>' +
                    '<div class="slider-sidebar__promo-video"> ' +
                    '<a class="slider-sidebar__promo-video-btn fancybox" href="#promovideo">&nbsp;</a> ' +
                    '<div class="slider-sidebar__promo-video-info">Подробнее о школе</div> ' +
                    '</div> ' +
                    '</div>}}' +
                    '</div>' +
                    '<!-- sidebar -->'
                },
                {
                    title: 'Везде и всегда',
                    description: '',
                    html: '<!-- content --> ' +
                    '<div class="slider-content"> ' +
                    '<div class="container"> ' +
                    '<div class="slider-content__inner-text"> ' +
                    '<h2>Везде <br>и всегда</h2> ' +
                    '<p>Занимайся йогой везде <br>и на любом устройстве</p> ' +
                    '</div> ' +
                    '</div> ' +
                    '</div> ' +
                    '<!-- content -->'
                },
                {
                    title: 'Первая онлайн школа йоги',
                    description: '',
                    html: '<!-- content --> ' +
                    '<div class="slider-content"> ' +
                    '<div class="container"> ' +
                    '<div class="slider-content__inner-text"> ' +
                    '<h2>Первая онлайн <br> школа йоги</h2> ' +
                    '<ul> ' +
                    '<li>Онлайн-семинары и трансляции</li> ' +
                    '<li>Авторские спецпроекты</li> ' +
                    '<li>Видео-уроки любого уровня сложности, <br>HD-качество съемки</li> ' +
                    '</ul> ' +
                    '</div> ' +
                    '</div> ' +
                    '</div> ' +
                    '<!-- content -->'
                },
                {
                    title: 'Опытные и известные преподаватели',
                    description: '',
                    html: '<!-- content --> ' +
                    '<div class="slider-content grey"> ' +
                    '<div class="container"> ' +
                    '<div class="slider-content__inner-text"> ' +
                    '<h2>Опытные <br>и известные <br>преподаватели</h2> ' +
                    '<p>Используй уникальную возможность <br> Занимайся с Мастерами йоги!</p> ' +
                    '</div> ' +
                    '</div> ' +
                    '</div> ' +
                    '<!-- content -->'
                },
                {
                    title: 'Опытные и известные преподаватели (белый цвет)',
                    description: '',
                    html: '<!-- content --> ' +
                    '<div class="slider-content"> ' +
                    '<div class="container"> ' +
                    '<div class="slider-content__inner-text"> ' +
                    '<h2>Опытные <br>и известные <br>преподаватели</h2> ' +
                    '<p>Используй уникальную возможность <br> Занимайся с Мастерами йоги!</p> ' +
                    '</div> ' +
                    '</div> ' +
                    '</div> ' +
                    '<!-- content -->'
                }
            ]
    }
);

CKEDITOR.addTemplates('seminar.attention',
    {
        templates :
            [
                {
                    title: 'Текст по умолчанию для предупреждения',
                    description: '',
                    html: '<p><span class="title">Друзья,</span><br /> ' +
                    'в случае недостаточного количества участников мы оставляем за собой право отменить трансляцию за два дня до мероприятия. В этом случае всем участникам трансляции сразу будет выслано уведомление об отмене. Возврат денежных средств осуществляется в полном объеме обратным переводом.</p> ' +
                    '<p class="selected">Заранее приобретая участие, вы не только экономите деньги, но и способствуете проведению трансляции.</p>' +
                    '<p><em>Благодарим за понимание.</em></p>'
                }
            ]
    }
);

CKEDITOR.addTemplates('seminar.additional',
    {
        templates :
            [
                {
                    title: 'Дополнительня информация в виде трех колонок',
                    description: '',
                    html: '<div class="description">' +
                    'Примерно за час до трансляции Вы получите письмо со ссылкой и паролем для входа. Трансляция начинается по московскому времени. ' +
                    '</div> ' +
                    '<div class="columns"> ' +
                    '<div class="column-time">' +
                    'Если вы произвели оплату менее, чем за час да трансляции или после ее начала, то рекомендуем связаться с нами через чат сайта или написать на почту <span class="selected">dayoga.info@gmail.com</span>, и сообщить об оплате. ' +
                    '</div> ' +
                    '<div class="column-dialog">' +
                    'Задать интересующие вас вопросы можно во время трансляции в режиме реального времени. Для этого можно использовать общий чат сайта или чат, который расположен непосредственно под окном видео. ' +
                    '</div> ' +
                    '<div class="column-video">' +
                    'Если вы произвели оплату менее, чем за час да трансляции или после ее начала, то рекомендуем связаться с нами через чат сайта или написать на почту dayoga.info@gmail.com, и сообщить об оплате. ' +
                    '</div> ' +
                    '</div>'
                }
            ]
    }
);